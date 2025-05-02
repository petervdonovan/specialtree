use extension_autobox::autobox;
use extension_everywhere_alternative::everywhere_alternative;
use extension_everywhere_maybemore::everywhere_maybemore;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortId, SortIdOf},
    sublang::Sublang,
    tymetafunc::Transparency,
};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, TyGenData, byline, proc_macro2};
use syn::parse_quote;

pub struct BasePaths {
    pub parse: syn::Path,
    pub data_structure: syn::Path,
    pub term_trait: syn::Path,
    pub words: syn::Path,
    pub pattern_match_strategy: syn::Path,
    pub cst_data_structure: syn::Path,
    pub cst_term_trait: syn::Path,
    pub cst_words: syn::Path,
}

pub fn generate<L: LangSpec>(bps: &BasePaths, lg: &LsGen<L>) -> syn::ItemMod {
    let byline = byline!();
    let arena = bumpalo::Bump::new();
    let cst = cst(&arena, lg.bak());
    let lg_cst = LsGen::from(cst);
    let cst_tgds = lg_cst.ty_gen_datas(None).collect::<Vec<_>>();
    let parsells = lg.ty_gen_datas(None).filter_map(|tgd| {
        tgd.id.as_ref().map(|_| {
            generate_parsell(
                bps,
                cst_tgds
                    .iter()
                    .find(|cst_tgd| cst_tgd.snake_ident == tgd.snake_ident)
                    .unwrap(),
            )
        })
    });
    let parses = lg.ty_gen_datas(None).filter_map(|tgd| {
        tgd.id.as_ref().map(|_| {
            generate_parse(
                bps,
                cst_tgds
                    .iter()
                    .find(|cst_tgd| cst_tgd.snake_ident == tgd.snake_ident)
                    .unwrap(),
            )
        })
    });
    let fnlut = generate_impl_fn_lut(
        bps,
        &lg_cst,
        cst.sublangs()
            .iter()
            .find(|it| &it.name == lg.bak().name())
            .unwrap(),
    );
    parse_quote! {
        #byline
        pub mod parse {
            #(#parses)*
            pub mod parsell {
                #(#parsells)*
            }
            #fnlut
        }
    }
}

pub(crate) fn generate_parse<LCst: LangSpec>(
    bps: &BasePaths,
    cst_tgd: &TyGenData<LCst>,
) -> syn::Item {
    let snake_ident = &cst_tgd.snake_ident;
    let cst_data_structure_bp = &bps.cst_data_structure;
    let my_cstfied_ty = cstfied_ty(&bps.cst_data_structure, cst_tgd);
    let pattern_match_strategy = &bps.pattern_match_strategy;
    let cvt = covisitable_trait(
        &bps.parse,
        pattern_match_strategy,
        cst_data_structure_bp,
        &syn::parse_quote! {'_},
    );
    let parse_bp = &bps.parse;
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub fn #snake_ident(source: &str) -> (#cst_data_structure_bp::Heap, #my_cstfied_ty) {
            let mut parser = parse_adt::Parser::new(source);
            let mut heap = #cst_data_structure_bp::Heap::default();
            let ret = <#my_cstfied_ty as #cvt>::co_visit(&mut parser, &mut heap, #parse_bp::fnlut::ParseWitness::new());
            (heap, ret)
        }
    }
}

fn cstfied_ty<LCst: LangSpec>(
    cst_data_structure_bp: &syn::Path,
    cst_tgd: &TyGenData<LCst>,
) -> syn::Type {
    let camel_ident = &cst_tgd.camel_ident;
    let transparency = &cst_tgd.transparency;
    let my_cst_ty: syn::Type = syn::parse_quote! { #cst_data_structure_bp::#camel_ident };
    let cstfication = transparency2cstfication(transparency);
    syn::parse_quote! {
        parse_adt::cstfy::#cstfication<
            #cst_data_structure_bp::Heap,
            #my_cst_ty,
        >
    }
}

fn covisitable_trait(
    parse_bp: &syn::Path,
    pattern_match_strategy: &syn::Path,
    cst_data_structure_bp: &syn::Path,
    lifetime: &syn::Lifetime,
) -> syn::Type {
    syn::parse_quote! {
        term::co_visit::CoVisitable<
            parse_adt::Parser<#lifetime, ()>,
            #pattern_match_strategy::PatternMatchStrategyProvider<#cst_data_structure_bp::Heap>,
            #cst_data_structure_bp::Heap,
            typenum::U8,
            #parse_bp::fnlut::ParseWitness<#lifetime>,
        >
    }
}

pub(crate) fn generate_impl_fn_lut<'a, 'b: 'a, LCst: LangSpec + 'b>(
    bps: &BasePaths,
    lg: &LsGen<LCst>,
    sublang: &Sublang<SortIdOf<LCst>>,
) -> syn::ItemMod {
    // let my_cstfied_ty = cstfied_ty(&bps.cst_data_structure, cst_tgd);
    let cvt = covisitable_trait(
        &bps.parse,
        &bps.pattern_match_strategy,
        &bps.cst_data_structure,
        &syn::parse_quote! {'c},
    );
    let cst_data_structure = &bps.cst_data_structure;
    let bp = &bps.parse;
    let idents = (0..).map(|i| syn::Ident::new(&format!("a{i}"), proc_macro2::Span::call_site()));
    let parsable_tys = lg
        .bak()
        .all_sort_ids()
        .filter(|sid| sublang.image.contains(sid))
        .map(|sid| {
            lg.sort2rs_ty(
                sid,
                &HeapType(syn::parse_quote! {#cst_data_structure::Heap}),
                &AlgebraicsBasePath::new(quote::quote! {#cst_data_structure::}),
            )
        });
    syn::parse_quote! {
        pub mod fnlut {
            term::impl_fn_lut!(
                witness_name ParseWitness <'c> ;
                trait #cvt ;
                fn_name co_visit ;
                get for <'a, 'b> fn(&'a mut parse_adt::Parser<'c, ()>, &'b mut #cst_data_structure::Heap, #bp::fnlut::ParseWitness<'c>) -> This ;
                types
                #(
                    #idents = #parsable_tys
                ),*
            );
        }
    }
}

pub fn generate_parsell<LCst: LangSpec>(bps: &BasePaths, cst_tgd: &TyGenData<LCst>) -> syn::Item {
    let camel_ident = &cst_tgd.camel_ident;
    let snake_ident = &cst_tgd.snake_ident;
    let cst_data_structure_bp = &bps.cst_data_structure;
    let my_cst_ty: syn::Type = syn::parse_quote! { #cst_data_structure_bp::#camel_ident };
    syn::parse_quote! {
        impl parse_adt::ParseLL for #my_cst_ty {
            const START: parse::KeywordSequence = parse::KeywordSequence(
                &[parse::Keyword::new(stringify!(#snake_ident))],
            );
            const PROCEED: &'static [parse::KeywordSequence] = &[];
            const END: parse::KeywordSequence = parse::KeywordSequence(
                &[],
            );
        }
    }
}

// pub fn gen_unparse(
//     snake_ident: &syn::Ident,
//     transparency: &Transparency,
//     cst_bp: &syn::Path,
//     my_ty: &syn::Type,
//     my_cst_ty: &syn::Type,
//     ccftys: &[syn::Type],
//     ccf_idxses: &[Vec<usize>],
// ) -> syn::ItemFn {
//     todo!()
// }

fn transparency2cstfication(transparency: &Transparency) -> syn::Ident {
    match transparency {
        Transparency::Transparent => syn::parse_quote! { CstfyTransparent },
        Transparency::Visible => syn::parse_quote! { Cstfy },
    }
}

pub(crate) fn cst<'a, 'b: 'a, L: LangSpec>(
    arena: &'a bumpalo::Bump,
    l: &'b L,
) -> &'a (impl LangSpec + 'a) {
    let errlang = arena.alloc(std_parse_error::parse_error());
    let fallible_ast = arena.alloc(everywhere_alternative(
        Name {
            human: "FallibleAst".into(),
            camel: "FallibleAst".into(),
            snake: "fallible_ast".into(),
        },
        l,
        errlang,
        SortId::TyMetaFunc(MappedType {
            f: std_parse_error::ParseErrorTmfId(),
            a: vec![],
        }),
    ));
    let parse_metadata = arena.alloc(std_parse_metadata::parse_metadata());
    arena.alloc(everywhere_maybemore(
        Name {
            human: "Cst".into(),
            camel: "Cst".into(),
            snake: "cst".into(),
        },
        fallible_ast,
        parse_metadata,
        SortId::TyMetaFunc(MappedType {
            f: std_parse_metadata::ParseMetadataTmfId(),
            a: vec![],
        }),
    ))
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec_gen_util::kebab_id;

    pub fn default<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let words =
                    codegen_deps.add(words::targets::words_mod(arena, codegen_deps.subtree(), l));
                let data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                let _ = codegen_deps.add(
                    term_pattern_match_strategy_provider_impl_gen::targets::words_impls(
                        arena,
                        codegen_deps.subtree(),
                        l,
                    ),
                );
                let term_trait = codegen_deps.add(term_trait_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                let pattern_match_strategy =
                    codegen_deps.add(term_pattern_match_strategy_provider_gen::targets::default(
                        arena,
                        codegen_deps.subtree(),
                        l,
                    ));
                let cst = super::cst(arena, arena.alloc(super::autobox(l)));
                let cst_data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    cst,
                ));
                let cst_term_trait = codegen_deps.add(term_trait_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    cst,
                ));
                let cst_words = codegen_deps.add(words::targets::words_mod(
                    arena,
                    codegen_deps.subtree(),
                    cst,
                ));
                let _ = codegen_deps.add(term_bridge_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    cst,
                    l,
                ));
                Box::new(move |c2sp, sp| {
                    let lg = super::LsGen::from(l);
                    super::generate(
                        &crate::BasePaths {
                            data_structure: data_structure(c2sp),
                            words: words(c2sp),
                            parse: sp,
                            term_trait: term_trait(c2sp),
                            pattern_match_strategy: pattern_match_strategy(c2sp),
                            cst_data_structure: cst_data_structure(c2sp),
                            cst_term_trait: cst_term_trait(c2sp),
                            cst_words: cst_words(c2sp),
                        },
                        &lg,
                    )
                })
            },
            external_deps: vec!["typenum"],
            workspace_deps: vec![
                ("parse-gen", Path::new(".")),
                ("std-parse-metadata", Path::new("langspec-std")),
                ("std-parse-error", Path::new("langspec-std")),
                ("term", Path::new(".")),
                ("parse-adt", Path::new(".")),
                ("parse", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
