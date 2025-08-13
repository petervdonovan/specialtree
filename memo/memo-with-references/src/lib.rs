use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn memo(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    memo_inner(args.into(), input.into())
        .map(|it| it.into())
        .unwrap_or_else(|e| e.into_compile_error().into())
}

fn memo_inner(attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
    let item_span = item.span();
    let item = syn::parse2::<syn::Item>(item)?;
    let mut itemfn = if let syn::Item::Fn(itemfn) = item {
        itemfn
    } else {
        Err(syn::Error::new(item_span, "expected fn item"))?
    };
    let cache_ident = get_cache_ident(attr)?;
    validate_no_mut_references(&itemfn)?;
    // Capture cache lifetime parameter for later return-type validation.
    let cache_lifetime = get_cache_lifetime(&cache_ident, &itemfn)?;
    let memoized_args = get_memoized_fn_args(&cache_ident, &itemfn);
    let cache_key_expr = get_cache_key(&memoized_args);
    let ogblock = itemfn.block; // original user body
    let cache_key_local_ident = syn::Ident::new("__memo_key", proc_macro2::Span::call_site());
    // We will generate code that:
    // 1. Builds key
    // 2. Attempts lookup
    // 3. On miss, runs body, inserts, returns value
    // Enforce that the function returns a reference with the same lifetime as the cache.
    let (ret_lookup_ty, ret_is_ok) = match &itemfn.sig.output {
        syn::ReturnType::Default => (syn::parse_quote! { () }, false),
        syn::ReturnType::Type(_, ty) => {
            match ty.as_ref() {
                syn::Type::Reference(r) => {
                    // Must be an immutable reference with matching lifetime.
                    let lifetime_matches = r
                        .lifetime
                        .as_ref()
                        .is_some_and(|lt| lt.ident == cache_lifetime.ident);
                    let not_mut = r.mutability.is_none();
                    let elem = (*r.elem).clone();
                    (elem, lifetime_matches && not_mut)
                }
                _ => ((*ty.as_ref()).clone(), false),
            }
        }
    };
    if !ret_is_ok {
        return Err(syn::Error::new(
            itemfn.sig.output.span(),
            format!(
                "memoized function return type must be an immutable reference with the same lifetime as cache (&{} T)",
                cache_lifetime.ident
            ),
        ));
    }
    itemfn.block = syn::parse_quote! {{
        let #cache_key_local_ident = #cache_key_expr;
        if let Some(hit) = #cache_ident.lookup::<#ret_lookup_ty>(&#cache_key_local_ident) {
            // Fast return on cache hit.
            return hit;
        }
        let __memo_result: & #cache_lifetime #ret_lookup_ty = (|| #ogblock )();
        let __memo_ref = #cache_ident.insert_ref(#cache_key_local_ident, __memo_result);
        __memo_ref
    }};
    Ok(quote::quote! {
        #itemfn
    })
}

fn get_cache_ident(attr: TokenStream) -> Result<proc_macro2::Ident, syn::Error> {
    let attr_span = attr.span();
    let mut it = attr.into_iter();
    let next = it.next();
    if it.next().is_some() {
        return Err(syn::Error::new(attr_span, "expected just a single ident"));
    }
    match next {
        Some(TokenTree::Ident(id)) => Ok(id),
        Some(_) => Err(syn::Error::new(attr_span, "expected an ident")),
        None => Err(syn::Error::new(
            attr_span,
            "expected token indicating which parameter is the memoization cache",
        )),
    }
}

fn get_cache_lifetime(
    ident: &proc_macro2::Ident,
    itemfn: &syn::ItemFn,
) -> Result<syn::Lifetime, syn::Error> {
    let ident_span = ident.span();
    for param in itemfn.sig.inputs.iter() {
        match param {
            syn::FnArg::Receiver(_) => {
                // Check if the cache parameter is supposed to be 'self'
                if ident == "self" {
                    return Err(syn::Error::new(ident_span, "cache param cannot be self"));
                }
                // Otherwise, skip self parameters
                continue;
            }
            syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                if let syn::Pat::Ident(pat_ident) = &**pat
                    && &pat_ident.ident == ident
                {
                    if let syn::Type::Reference(r) = &**ty {
                        match &r.lifetime {
                            Some(lt) => return Ok(lt.clone()),
                            None => {
                                return Err(syn::Error::new(
                                    r.span(),
                                    "explicit lifetime required on cache argument span",
                                ));
                            }
                        }
                    } else {
                        return Err(syn::Error::new(
                            ty.span(),
                            "expected reference type with an explicit lifetime",
                        ));
                    }
                }
            }
        }
    }
    Err(syn::Error::new(
        ident_span,
        format!("fn parameter named {ident} not found"),
    ))
}

fn validate_no_mut_references(itemfn: &syn::ItemFn) -> Result<(), syn::Error> {
    for param in itemfn.sig.inputs.iter() {
        match param {
            syn::FnArg::Receiver(receiver) => {
                if receiver.mutability.is_some() {
                    return Err(syn::Error::new(
                        receiver.span(),
                        "memoizable functions cannot have mutable self parameter",
                    ));
                }
            }
            syn::FnArg::Typed(syn::PatType { ty, .. }) => {
                if let syn::Type::Reference(r) = &**ty
                    && r.mutability.is_some()
                {
                    return Err(syn::Error::new(
                        r.span(),
                        "memoizable functions cannot have mutable reference parameters",
                    ));
                }
            }
        }
    }
    Ok(())
}

struct MemoizedFnArgs {
    frozen: Box<[proc_macro2::Ident]>,
    cloneable: Box<[proc_macro2::Ident]>,
}

fn get_memoized_fn_args(cache_ident: &proc_macro2::Ident, itemfn: &syn::ItemFn) -> MemoizedFnArgs {
    let mut frozen = Vec::new();
    let mut cloneable = Vec::new();

    for param in itemfn.sig.inputs.iter() {
        match param {
            syn::FnArg::Receiver(receiver) => {
                // Create an ident for 'self'
                let self_ident = proc_macro2::Ident::new("self", receiver.span());

                // Categorize based on whether it's &self (frozen) or self (cloneable)
                if receiver.reference.is_some() {
                    // &self or &mut self - frozen (reference type)
                    frozen.push(self_ident);
                } else {
                    // self - cloneable (owned type)
                    cloneable.push(self_ident);
                }
            }
            syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                if let syn::Pat::Ident(pat_ident) = &**pat {
                    // Skip the cache parameter
                    if &pat_ident.ident == cache_ident {
                        continue;
                    }

                    // Check if it's a reference type
                    if let syn::Type::Reference(_) = &**ty {
                        frozen.push(pat_ident.ident.clone());
                    } else {
                        cloneable.push(pat_ident.ident.clone());
                    }
                }
            }
        }
    }

    MemoizedFnArgs {
        frozen: frozen.into_boxed_slice(),
        cloneable: cloneable.into_boxed_slice(),
    }
}

fn get_pointer_fingerprint(memoized_args: &MemoizedFnArgs) -> syn::Expr {
    let mut pointer_elements = Vec::new();

    // Add raw pointers for frozen parameters
    for frozen_param in memoized_args.frozen.iter() {
        let raw_pointer_expr: syn::Expr = syn::parse_quote! {
            #frozen_param as *const _
        };
        pointer_elements.push(raw_pointer_expr);
    }

    // Create the tuple expression for pointers
    let pointer_tuple: syn::Expr = if pointer_elements.is_empty() {
        // Empty tuple for functions with no frozen parameters
        syn::parse_quote! { () }
    } else if pointer_elements.len() == 1 {
        // Single element - wrap in tuple syntax to ensure it's a tuple
        let element = &pointer_elements[0];
        syn::parse_quote! { (#element,) }
    } else {
        // Multiple elements - create tuple
        syn::parse_quote! { (#(#pointer_elements),*) }
    };

    // Generate code that creates a SHA-256 hash of the pointer tuple
    syn::parse_quote! {
        {
            use memo_cache::sha2::{Sha256, Digest};
            use std::hash::{Hash, Hasher};
            use std::collections::hash_map::DefaultHasher;

            let pointer_tuple = #pointer_tuple;

            // First, get a standard hash of the tuple
            let mut hasher = DefaultHasher::new();
            pointer_tuple.hash(&mut hasher);
            let hash64 = hasher.finish();

            // Create a SHA-256 hash for more robust fingerprinting
            let mut sha_hasher = Sha256::new();
            sha_hasher.update(hash64.to_le_bytes());
            let result = sha_hasher.finalize();
            let mut first16 = [0u8; 16];
            first16.copy_from_slice(&result[..16]);
            u128::from_le_bytes(first16)
        }
    }
}

fn get_cache_key(memoized_args: &MemoizedFnArgs) -> syn::Expr {
    let pointer_fingerprint = get_pointer_fingerprint(memoized_args);
    let cloneable_tuple = get_cloneable_tuple(memoized_args);

    // Generate code that creates a MemoCacheKey with both fingerprint and cloneable data
    syn::parse_quote! {
        memo_cache::MemoCacheKey::new(#pointer_fingerprint, #cloneable_tuple)
    }
}

fn get_cloneable_tuple(memoized_args: &MemoizedFnArgs) -> syn::Expr {
    let mut cloneable_elements = Vec::new();

    // Add clones for cloneable parameters
    for cloneable_param in memoized_args.cloneable.iter() {
        let clone_expr: syn::Expr = syn::parse_quote! {
            #cloneable_param.clone()
        };
        cloneable_elements.push(clone_expr);
    }

    // Create the tuple expression for cloneable values
    if cloneable_elements.is_empty() {
        // Empty tuple for functions with no cloneable parameters
        syn::parse_quote! { () }
    } else if cloneable_elements.len() == 1 {
        // Single element - wrap in tuple syntax to ensure it's a tuple
        let element = &cloneable_elements[0];
        syn::parse_quote! { (#element,) }
    } else {
        // Multiple elements - create tuple
        syn::parse_quote! { (#(#cloneable_elements),*) }
    }
}
