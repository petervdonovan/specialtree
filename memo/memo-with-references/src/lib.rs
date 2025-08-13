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
    // Parse and validate all inputs before any mutations
    let item_span = item.span();
    let item = syn::parse2::<syn::Item>(item)?;
    let itemfn = if let syn::Item::Fn(itemfn) = item {
        itemfn
    } else {
        return Err(syn::Error::new(item_span, "expected fn item"));
    };

    let cache_lifetime = get_cache_lifetime(attr)?;
    validate_no_mut_references(&itemfn)?;
    let return_type_info = analyze_return_type(&itemfn.sig.output, &cache_lifetime)?;
    let memoized_args = get_original_memoized_fn_args(&cache_lifetime, &itemfn);

    // Now mutate to build the final function
    let mut final_fn = itemfn;
    add_cache_parameter(&mut final_fn.sig, &cache_lifetime);
    update_return_type(&mut final_fn.sig, &return_type_info, &cache_lifetime);
    let new_body = generate_memo_body(
        &final_fn.block,
        &cache_key_expr(&memoized_args),
        &return_type_info,
        &cache_lifetime,
    );
    final_fn.block = Box::new(new_body);

    Ok(quote::quote! { #final_fn })
}

#[derive(Debug)]
struct ReturnTypeInfo {
    lookup_type: syn::Type,
    needs_ref_wrapper: bool,
}

fn analyze_return_type(
    output: &syn::ReturnType,
    cache_lifetime: &syn::Lifetime,
) -> Result<ReturnTypeInfo, syn::Error> {
    match output {
        syn::ReturnType::Default => {
            // Default () return - will need wrapper
            Ok(ReturnTypeInfo {
                lookup_type: syn::parse_quote! { () },
                needs_ref_wrapper: true,
            })
        }
        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            syn::Type::Reference(r) => {
                // Already a reference - check if lifetime matches
                let lifetime_matches = r
                    .lifetime
                    .as_ref()
                    .is_some_and(|lt| lt.ident == cache_lifetime.ident);
                let not_mut = r.mutability.is_none();

                if !lifetime_matches || !not_mut {
                    return Err(syn::Error::new(
                        ty.span(),
                        format!(
                            "memoized function return type must be an immutable reference with the cache lifetime (&{} T)",
                            cache_lifetime.ident
                        ),
                    ));
                }

                Ok(ReturnTypeInfo {
                    lookup_type: (*r.elem).clone(),
                    needs_ref_wrapper: false,
                })
            }
            _ => {
                // Not a reference - will need wrapper
                Ok(ReturnTypeInfo {
                    lookup_type: (*ty.as_ref()).clone(),
                    needs_ref_wrapper: true,
                })
            }
        },
    }
}

fn get_original_memoized_fn_args(
    cache_lifetime: &syn::Lifetime,
    itemfn: &syn::ItemFn,
) -> MemoizedFnArgs {
    let mut frozen = Vec::new();
    let mut cloneable = Vec::new();

    // Process original parameters (before cache injection)
    for param in itemfn.sig.inputs.iter() {
        match param {
            syn::FnArg::Receiver(receiver) => {
                let self_ident = proc_macro2::Ident::new("self", receiver.span());
                if receiver.reference.is_some() {
                    frozen.push(self_ident);
                } else {
                    cloneable.push(self_ident);
                }
            }
            syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                if let syn::Pat::Ident(pat_ident) = &**pat {
                    if let syn::Type::Reference(r) = &**ty {
                        let ref_lifetime_matches = r
                            .lifetime
                            .as_ref()
                            .is_some_and(|lt| lt.ident == cache_lifetime.ident);

                        if ref_lifetime_matches {
                            frozen.push(pat_ident.ident.clone());
                        } else {
                            cloneable.push(pat_ident.ident.clone());
                        }
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

fn add_cache_parameter(sig: &mut syn::Signature, cache_lifetime: &syn::Lifetime) {
    // Add a generic logger type parameter
    let logger_generic: syn::GenericParam = syn::parse_quote! { Logger: memo_cache::MemoCacheLogger };
    sig.generics.params.push(logger_generic);
    
    let cache_param: syn::FnArg = syn::parse_quote! {
        cache: &#cache_lifetime memo_cache::Cache<#cache_lifetime, Logger>
    };
    sig.inputs.insert(0, cache_param);
}

fn update_return_type(
    sig: &mut syn::Signature,
    return_info: &ReturnTypeInfo,
    cache_lifetime: &syn::Lifetime,
) {
    if return_info.needs_ref_wrapper {
        let lookup_type = &return_info.lookup_type;
        sig.output = syn::parse_quote! { -> &#cache_lifetime #lookup_type };
    }
}

fn cache_key_expr(memoized_args: &MemoizedFnArgs) -> syn::Expr {
    get_cache_key(memoized_args)
}

fn generate_memo_body(
    original_body: &syn::Block,
    cache_key_expr: &syn::Expr,
    return_info: &ReturnTypeInfo,
    cache_lifetime: &syn::Lifetime,
) -> syn::Block {
    let cache_key_ident = syn::Ident::new("__memo_key", proc_macro2::Span::call_site());
    let lookup_type = &return_info.lookup_type;

    let insert_or_insert_ref = if return_info.needs_ref_wrapper {
        quote::quote! { insert }
    } else {
        quote::quote! { insert_ref }
    };
    syn::parse_quote! {{
        let #cache_key_ident = #cache_key_expr;
        if let Some(hit) = cache.lookup::<#lookup_type>(&#cache_key_ident) {
            return hit;
        }
        let __memo_result = #original_body;
        let __memo_ref = cache.#insert_or_insert_ref(#cache_key_ident, __memo_result);
        __memo_ref
    }}
}

fn get_cache_lifetime(attr: TokenStream) -> Result<syn::Lifetime, syn::Error> {
    let attr_span = attr.span();
    // let tokens: Vec<_> = attr.iter().collect();

    // Expect exactly two tokens: apostrophe and identifier
    if attr.is_empty() {
        return Err(syn::Error::new(
            attr_span,
            "expected lifetime in #[memo('a)]",
        ));
    }
    // else if tokens.len() != 2 {
    //     return Err(syn::Error::new(
    //         attr_span,
    //         "expected at most single lifetime: #[memo('a)]",
    //     ));
    // }

    // match (&tokens[0], &tokens[1]) {
    //     (TokenTree::Punct(p), TokenTree::Ident(id)) if p.as_char() == '\'' => {
    //         let lifetime_str = format!("'{id}");
    //         Ok(syn::Lifetime::new(
    //             &lifetime_str,
    //             p.span().join(id.span()).unwrap_or(p.span()),
    //         ))
    //     }
    //     _ => Err(syn::Error::new(
    //         attr_span,
    //         "expected lifetime identifier inside #[memo(..)]",
    //     )),
    // }
    syn::parse2(attr)
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
