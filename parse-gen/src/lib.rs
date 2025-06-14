use extension_autobox::autobox;
use extension_everywhere_alternative::everywhere_alternative;
use extension_everywhere_maybemore::everywhere_maybemore;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortId},
    tymetafunc::Transparency,
};
use langspec_gen_util::{LsGen, TyGenData, byline};
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
    let parses = lg.ty_gen_datas(None).map(|tgd| {
        generate_parse(
            bps,
            cst_tgds
                .iter()
                .find(|cst_tgd| cst_tgd.snake_ident == tgd.snake_ident)
                .unwrap(),
        )
    });
    parse_quote! {
        #byline
        pub mod parse {
            #(#parses)*
        }
    }
}

pub(crate) fn generate_parse<LCst: LangSpec>(
    bps: &BasePaths,
    cst_tgd: &TyGenData<LCst>,
) -> syn::Item {
    let snake_ident = &cst_tgd.snake_ident;
    let camel_ident = &cst_tgd.camel_ident;
    let cst_data_structure_bp = &bps.cst_data_structure;
    let my_cstfied_ty = cstfied_ty(&bps.cst_data_structure, cst_tgd);
    let byline = byline!();
    let words = &bps.words;
    syn::parse_quote! {
        #byline
        pub fn #snake_ident(source: &str) -> (#cst_data_structure_bp::Heap, #my_cstfied_ty) {
            let mut parser = parse_adt::Parser::new(source);
            let mut heap = #cst_data_structure_bp::Heap::default();
            let ret = <parse_adt::Parser<'_, #words::L> as covisit::Covisit<#words::sorts::#camel_ident, #words::L, #my_cstfied_ty, #cst_data_structure_bp::Heap, words::AdtLike>>::covisit(&mut parser, &mut heap);
            (heap, ret)
        }
    }
}

fn cstfied_ty<LCst: LangSpec>(
    cst_data_structure_bp: &syn::Path,
    cst_tgd: &TyGenData<LCst>,
) -> syn::Type {
    let camel_ident = &cst_tgd.camel_ident;
    let my_cst_ty: syn::Type = syn::parse_quote! { #cst_data_structure_bp::#camel_ident };
    syn::parse_quote! {
        parse_adt::cstfy::Cstfy<
            #cst_data_structure_bp::Heap,
            #my_cst_ty,
        >
    }
}

pub(crate) fn generate_parsell<L: LangSpec, LCst: LangSpec>(
    og_words_bp: &syn::Path,
    cst_tgd: &TyGenData<LCst>,
    tgd: &TyGenData<L>,
) -> syn::Item {
    let camel_ident = &cst_tgd.camel_ident;
    let snake_ident = &cst_tgd.snake_ident;
    let byline = byline!();
    if tgd.transparency == Transparency::Transparent {
        syn::parse_quote! {
            #byline
            impl parse_adt::NamesParseLL for #og_words_bp::sorts::#camel_ident {
                const START: parse::KeywordSequence = parse::KeywordSequence(
                    &[],
                );
                const PROCEED: &'static [parse::KeywordSequence] = &[];
                const END: parse::KeywordSequence = parse::KeywordSequence(
                    &[],
                );
            }
        }
    } else {
        syn::parse_quote! {
            #byline
            impl parse_adt::NamesParseLL for #og_words_bp::sorts::#camel_ident {
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
}

pub fn cst<'a, 'b: 'a, L: LangSpec>(
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
        }
        .merge(l.name()),
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
    use extension_autobox::autobox;
    use langspec::{
        langspec::{LangSpec as _, SortIdOf},
        sublang::{SublangsList, reflexive_sublang},
    };
    use langspec_gen_util::kebab_id;

    pub fn default<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
        other_sublangs: impl SublangsList<'langs, SortIdOf<L>> + 'langs,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let l_autobox = arena.alloc(autobox(l));
                let words =
                    codegen_deps.add(words::targets::words_mod(arena, codegen_deps.subtree(), l));
                let data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l_autobox,
                ));
                let _ = codegen_deps.add(term_specialized_impl_gen::targets::words_impls(
                    arena,
                    codegen_deps.subtree(),
                    l_autobox,
                    arena.alloc(l_autobox.sublang(l).unwrap()),
                ));
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
                let _ =
                    codegen_deps.add(term_pattern_match_strategy_provider_gen::targets::default(
                        arena,
                        codegen_deps.subtree(),
                        cst,
                    ));
                let _ = codegen_deps.add(
                    term_pattern_match_strategy_provider_impl_gen::targets::default(
                        arena,
                        codegen_deps.subtree(),
                        cst,
                    ),
                );
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
                let sublang = cst.sublang(l).unwrap();
                let _ = codegen_deps.add(term_bridge_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    cst,
                    arena.alloc((
                        reflexive_sublang(cst),
                        (sublang, other_sublangs.push_through(cst)),
                    )),
                ));
                // let _ = codegen_deps.add(term_pattern_match_strategy_provider_impl_gen::targets::uses_strategy_for_traversal_impls(arena, codegen_deps.subtree(), cst, &[cst.name().clone(), l.name().clone()]));
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
                    // syn::parse_quote! {pub mod temp {}}
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
                ("covisit", Path::new(".")),
            ],
            codegen_deps,
        }
    }
    pub fn parsells<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let byline = super::byline!();
                let words =
                    codegen_deps.add(words::targets::words_mod(arena, codegen_deps.subtree(), l));
                Box::new(move |c2sp, _| {
                    let lg = super::LsGen::from(l);
                    let cst = super::cst(arena, l);
                    let lg_cst = super::LsGen::from(cst);
                    let cst_tgds = lg_cst.ty_gen_datas(None).collect::<Vec<_>>();
                    let parsells = lg.ty_gen_datas(None).map(|tgd| {
                        super::generate_parsell(
                            &words(c2sp),
                            cst_tgds
                                .iter()
                                .find(|cst_tgd| cst_tgd.snake_ident == tgd.snake_ident)
                                .unwrap(),
                            &tgd,
                        )
                    });
                    syn::parse_quote! {
                        #byline
                        pub mod parsell {
                            #(#parsells)*
                        }
                    }
                })
            },
            external_deps: vec!["typenum"],
            workspace_deps: vec![
                ("parse-gen", Path::new(".")),
                ("parse", Path::new(".")),
                ("parse-adt", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
