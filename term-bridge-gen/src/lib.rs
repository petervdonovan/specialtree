use langspec::{
    langspec::{LangSpec, Name, SortIdOf},
    sublang::Sublang,
};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen};

pub struct BasePaths {
    pub ext_data_structure: syn::Path,
    pub og_term_trait: syn::Path,
}

pub fn generate<L: LangSpec>(
    ext_lg: &LsGen<L>,
    sublang_name: &Name,
    bps: &BasePaths,
) -> syn::ItemImpl {
    let sublang = ext_lg
        .bak()
        .sublangs()
        .into_iter()
        .find(|sublang| sublang.name == *sublang_name)
        .expect("Sublang not found");
    generate_heap(ext_lg, &sublang, bps)
}

pub(crate) fn generate_heap<L: LangSpec>(
    ext_lg: &LsGen<L>,
    sublang: &Sublang<SortIdOf<L>>,
    BasePaths {
        ext_data_structure,
        og_term_trait,
    }: &BasePaths,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    let camel_names = sublang
        .ty_names
        .iter()
        .map(|name| syn::Ident::new(&name.camel, proc_macro2::Span::call_site()));
    let image_ty_under_embeddings = sublang.ty_names.iter().map(&sublang.map).map(|sort| {
        ext_lg.sort2rs_ty(
            sort,
            &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
            &AlgebraicsBasePath::new(quote::quote! {#ext_data_structure::}),
        )
    });
    syn::parse_quote! {
        #byline
        impl #og_term_trait::Heap for #ext_data_structure::Heap {
            #(
                type #camel_names = #image_ty_under_embeddings;
            )*
            // type F = parse_adt::cstfy::Cstfy<
            //     crate::cst::data_structure::Heap,
            //     crate::cst::data_structure::F,
            // >;
            // type Plus = parse_adt::cstfy::Cstfy<
            //     crate::cst::data_structure::Heap,
            //     crate::cst::data_structure::Plus,
            // >;
            // type LeftOperand = parse_adt::cstfy::Cstfy<
            //     crate::cst::data_structure::Heap,
            //     crate::cst::data_structure::LeftOperand,
            // >;
            // type RightOperand = parse_adt::cstfy::Cstfy<
            //     crate::cst::data_structure::Heap,
            //     crate::cst::data_structure::RightOperand,
            // >;
            // type Sum = parse_adt::cstfy::Cstfy<
            //     crate::cst::data_structure::Heap,
            //     crate::cst::data_structure::Sum,
            // >;
            // type Nat = parse_adt::cstfy::CstfyTransparent<
            //     crate::cst::data_structure::Heap,
            //     crate::cst::data_structure::Nat,
            // >;
        }
    }
}
