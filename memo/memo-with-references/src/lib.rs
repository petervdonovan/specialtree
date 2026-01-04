use proc_macro2::TokenStream;
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
    let itemfn = if let syn::Item::Fn(itemfn) = item {
        itemfn
    } else {
        return Err(syn::Error::new(item_span, "expected fn item"));
    };

    let cache_lifetime = get_cache_lifetime(attr)?;
    validate_no_mut_references(&itemfn)?;
    let return_type_info = analyze_return_type(&itemfn.sig.output, &cache_lifetime)?;
    let memoized_args = get_original_memoized_fn_args(&cache_lifetime, &itemfn);

    let original_sig = itemfn.sig.clone();

    let mut final_fn = itemfn;
    add_cache_parameter(&mut final_fn.sig, &cache_lifetime);
    add_where_bounds_for_cloneable_params(&mut final_fn.sig, &memoized_args);
    update_return_type(&mut final_fn.sig, &return_type_info, &cache_lifetime);
    let new_body = generate_memo_body(
        &final_fn.block,
        &cache_key_expr(&memoized_args, &original_sig, item_span),
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
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "memoized functions cannot have unit return type - there is no value to cache",
            ));
        }
        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            syn::Type::Tuple(tuple) if tuple.elems.is_empty() => {
                return Err(syn::Error::new(
                    ty.span(),
                    "memoized functions cannot have unit return type - there is no value to cache",
                ));
            }
            syn::Type::Reference(r) => {
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
            _ => Ok(ReturnTypeInfo {
                lookup_type: (*ty.as_ref()).clone(),
                needs_ref_wrapper: true,
            }),
        },
    }
}

fn get_original_memoized_fn_args(
    cache_lifetime: &syn::Lifetime,
    itemfn: &syn::ItemFn,
) -> MemoizedFnArgs {
    let mut frozen = Vec::new();
    let mut cloneable = Vec::new();

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
    let logger_generic: syn::GenericParam =
        syn::parse_quote! { Logger: memo::memo_cache::logger::MemoCacheLogger };
    sig.generics.params.push(logger_generic);

    let cache_param: syn::FnArg = syn::parse_quote! {
        cache: &#cache_lifetime memo::memo_cache::Cache<#cache_lifetime, Logger>
    };

    let insert_pos = if sig
        .inputs
        .first()
        .is_some_and(|arg| matches!(arg, syn::FnArg::Receiver(_)))
    {
        1
    } else {
        0
    };

    sig.inputs.insert(insert_pos, cache_param);
}

fn add_where_bounds_for_cloneable_params(sig: &mut syn::Signature, memoized_args: &MemoizedFnArgs) {
    let mut types_needing_bounds = Vec::new();

    for param in sig.inputs.iter() {
        match param {
            syn::FnArg::Receiver(receiver) => {
                if receiver.reference.is_none() {
                    let self_type: syn::Type = syn::parse_quote! { Self };
                    types_needing_bounds.push(self_type);
                }
            }
            syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                if let syn::Pat::Ident(pat_ident) = &**pat
                    && memoized_args
                        .cloneable
                        .iter()
                        .any(|c| c == &pat_ident.ident)
                {
                    let bound_type = extract_type_for_bounds(ty);
                    types_needing_bounds.push(bound_type);
                }
            }
        }
    }

    for bound_type in types_needing_bounds {
        add_cache_key_trait_bounds(sig, &bound_type);
    }
}

fn extract_type_for_bounds(ty: &syn::Type) -> syn::Type {
    match ty {
        syn::Type::Reference(type_ref) => (*type_ref.elem).clone(),
        _ => ty.clone(),
    }
}

fn add_cache_key_trait_bounds(sig: &mut syn::Signature, ty: &syn::Type) {
    let where_clause = sig
        .generics
        .where_clause
        .get_or_insert_with(|| syn::WhereClause {
            where_token: syn::Token![where](proc_macro2::Span::call_site()),
            predicates: syn::punctuated::Punctuated::new(),
        });

    let clone_bound: syn::WherePredicate = syn::parse_quote! {
        #ty: Clone
    };
    let eq_bound: syn::WherePredicate = syn::parse_quote! {
        #ty: Eq
    };
    let hash_bound: syn::WherePredicate = syn::parse_quote! {
        #ty: std::hash::Hash
    };

    where_clause.predicates.push(clone_bound);
    where_clause.predicates.push(eq_bound);
    where_clause.predicates.push(hash_bound);
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

fn cache_key_expr(
    memoized_args: &MemoizedFnArgs,
    fn_signature: &syn::Signature,
    span: proc_macro2::Span,
) -> syn::Expr {
    get_cache_key(memoized_args, fn_signature, span)
}

fn generate_memo_body(
    original_body: &syn::Block,
    cache_key_expr: &syn::Expr,
    return_info: &ReturnTypeInfo,
    _cache_lifetime: &syn::Lifetime,
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
    if attr.is_empty() {
        return Err(syn::Error::new(
            attr_span,
            "expected lifetime in #[memo('a)]",
        ));
    }
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

fn get_pointer_fingerprint(
    memoized_args: &MemoizedFnArgs,
    _fn_signature: &syn::Signature,
    _span: proc_macro2::Span,
) -> syn::Expr {
    let hasher_updates: Vec<syn::Block> = memoized_args
        .frozen
        .iter()
        .map(|arg| {
            syn::parse_quote! {{
                sha_hasher.update(((#arg as *const _) as u64).to_le_bytes());
            }}
        })
        .collect();

    syn::parse_quote! {
        {
            use memo::memo_cache::sha2::{Sha256, Digest};
            let mut sha_hasher = Sha256::new();

            // Include pointer values for frozen arguments only
            #(
                #hasher_updates
            )*

            let result = sha_hasher.finalize();
            let mut first16 = [0u8; 16];
            first16.copy_from_slice(&result[..16]);
            u128::from_le_bytes(first16)
        }
    }
}

fn get_cache_key(
    memoized_args: &MemoizedFnArgs,
    fn_signature: &syn::Signature,
    span: proc_macro2::Span,
) -> syn::Expr {
    let pointer_fingerprint = get_pointer_fingerprint(memoized_args, fn_signature, span);
    let cloneable_tuple = get_cloneable_tuple(memoized_args);

    // Generate code that creates a MemoCacheKey with both fingerprint and cloneable data
    syn::parse_quote! {
        memo::memo_cache::MemoCacheKey::new(#pointer_fingerprint, #cloneable_tuple)
    }
}

fn get_cloneable_tuple(memoized_args: &MemoizedFnArgs) -> syn::Expr {
    let mut cloneable_elements = Vec::new();

    // Add a closure TypeId to capture the current monomorphization
    // The closure will have a different TypeId for each monomorphization
    let closure_typeid_expr: syn::Expr = syn::parse_quote! {
        {
            use std::any::Any;
            (|| ()).type_id()
        }
    };
    cloneable_elements.push(closure_typeid_expr);

    // Add clones for cloneable parameters
    for cloneable_param in memoized_args.cloneable.iter() {
        let clone_expr: syn::Expr = syn::parse_quote! {
            #cloneable_param.clone()
        };
        cloneable_elements.push(clone_expr);
    }

    // Create the tuple expression for cloneable values
    if cloneable_elements.len() == 1 {
        // Single element - wrap in tuple syntax to ensure it's a tuple
        let element = &cloneable_elements[0];
        syn::parse_quote! { (#element,) }
    } else {
        // Multiple elements - create tuple
        syn::parse_quote! { (#(#cloneable_elements),*) }
    }
}
