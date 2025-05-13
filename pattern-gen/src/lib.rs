use langspec::langspec::{LangSpec, SortIdOf, ToLiteral as _};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, byline, proc_macro2};
use pattern_dyn::{DynPattern, NamedDynPattern};

#[allow(type_alias_bounds)]
type NamedPatternOf<L: LangSpec> = NamedDynPattern<SortIdOf<L>>;

pub fn generate<L: LangSpec>(lg: &LsGen<L>, patterns: &[NamedPatternOf<L>]) -> syn::ItemMod {
    let byline = byline!();
    let literals = lg.bak().all_sort_ids().map(|it| it.to_literal());
    let names = lg.bak().all_sort_ids().map(|sid| {
        lg.sort2rs_ty(
            sid,
            &HeapType(syn::parse_quote! {Heap}),
            &AlgebraicsBasePath::new(syn::parse_quote! {abp::}),
        )
    });
    let named_patterns = patterns.iter().map(|it| generate_named_pattern(lg, it));
    syn::parse_quote! {
        #byline
        #[doc = stringify!(#(#names = #literals),*)]
        pub mod patterns {
            #(#named_patterns)*
        }
    }
}

pub(crate) fn generate_named_pattern<L: LangSpec>(
    lg: &LsGen<L>,
    np: &NamedPatternOf<L>,
) -> syn::ItemMod {
    let mod_name: syn::Ident = syn::Ident::new(&np.snake_ident, proc_macro2::Span::call_site());
    generate_pattern(lg, &np.pattern, mod_name)
}

pub(crate) fn generate_pattern<L: LangSpec>(
    lg: &LsGen<L>,
    p: &DynPattern<SortIdOf<L>>,
    mod_name: syn::Ident,
) -> syn::ItemMod {
    syn::parse_quote! {
        mod #mod_name {

        }
    }
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec_gen_util::{LsGen, kebab_id};

    pub fn default<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
        patterns: &Path,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let patterns = vec![];
                let lg = LsGen::from(l);
                Box::new(move |c2sp, p| super::generate(&lg, patterns.as_slice()))
            },
            external_deps: vec![],
            workspace_deps: vec![("pattern-gen", Path::new(".")), ("pattern", Path::new("."))],
            codegen_deps,
        }
    }
}
