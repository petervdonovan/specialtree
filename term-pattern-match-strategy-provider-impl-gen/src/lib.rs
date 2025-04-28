use extension_autobox::autobox;
use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, TerminalLangSpec},
    tymetafunc::TyMetaFuncSpec,
};
use langspec_gen_util::LsGen;

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub words: syn::Path,
    pub strategy_provider: syn::Path,
}

pub fn generate<L: LangSpec>(base_paths: &BasePaths, ls: &LsGen<L>) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    let impls = ls.ty_gen_datas(Some(base_paths.words.clone())).map(|tgd| {
        let camel_ident = tgd.camel_ident;
        impl_adt_for(base_paths, &camel_ident)
    });
    syn::parse_quote! {
        #byline
        pub mod pattern_match_strategy_impls {
            #(#impls)*
        }
    }
}

pub(crate) fn impl_adt_for(
    BasePaths {
        data_structure,
        words: _,
        strategy_provider,
    }: &BasePaths,
    camel_ident: &syn::Ident,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        impl term::case_split::Adt for #data_structure::#camel_ident {
            type PatternMatchStrategyProvider = #strategy_provider::PatternMatchStrategyProvider<#data_structure::Heap>;
        }
    }
}

pub fn formatted<Tmfs: TyMetaFuncSpec>(lsh: &LangSpecHuman<Tmfs>) -> String {
    let lsf: LangSpecFlat<Tmfs> = LangSpecFlat::canonical_from(lsh);
    let lsf_box = autobox(&lsf);
    let lsg = LsGen::from(&lsf);
    let lsg_box = LsGen::from(&lsf_box);
    let term_trait_bp = syn::parse_quote!(crate::term_trait);
    let bps = BasePaths {
        strategy_provider: syn::parse_quote!(crate::pattern_match_strategy),
        data_structure: syn::parse_quote!(crate::data_structure),
        words: syn::parse_quote!(crate::term_trait::words),
    };
    let m = generate(&bps, &lsg);
    let tt = term_trait_gen::generate(&term_trait_bp, &lsf);
    let ds = term_specialized_gen::generate(&bps.data_structure, &lsg_box, false);
    let words_impls = words::words_impls(&bps.words, &bps.data_structure, &lsg);
    let tpmsp = term_pattern_match_strategy_provider_gen::generate(
        &term_pattern_match_strategy_provider_gen::BasePaths {
            term_trait: term_trait_bp.clone(),
            data_structure: bps.data_structure.clone(),
            words: bps.words.clone(),
            strategy_provider: bps.strategy_provider.clone(),
        },
        &lsg,
    );
    let ttimpl = term_specialized_impl_gen::generate(
        &term_specialized_impl_gen::BasePaths {
            data_structure: bps.data_structure.clone(),
            term_trait: term_trait_bp.clone(),
        },
        &lsg_box,
        &[lsf.name().clone()],
    );
    prettyplease::unparse(&syn_insert_use::insert_use(syn::parse_quote! {
        #m
        #tpmsp
        #tt
        #ds
        #words_impls
        #ttimpl
    }))
}
