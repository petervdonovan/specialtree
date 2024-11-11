use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, TerminalLangSpec as _},
};
use langspec_gen_util::LangSpecGen;

pub struct BasePaths {
    pub refbased_data_structure: syn::Path,
    pub extension_of: syn::Path,
}

pub fn gen<L: LangSpec>(
    BasePaths {
        refbased_data_structure,
        extension_of,
    }: &BasePaths,
    lg: LangSpecGen<L>,
) -> syn::ItemFn {
    let ls_camel_ident = lg.camel_ident();
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        fn parse<T: #extension_of::owned::#ls_camel_ident>(input: &str) -> Result<(<T as #extension_of::#ls_camel_ident>::LImpl, T), ()> {
            let refbased: #refbased_data_structure::#ls_camel_ident = serde_yml::from_str(input).map_err(|_| ())?;
            let l = core::default::Default::default();
            let lo = core::default::Default::default();
            let ret = refbased.convert(&mut lo);
            Ok((lo, ret))
        }
    }
}

pub fn formatted(lsh: &LangSpecHuman) -> String {
    let lsf: LangSpecFlat = LangSpecFlat::canonical_from(lsh);
    let bps = BasePaths {
        extension_of: syn::parse_quote!(crate::extension_of),
        refbased_data_structure: syn::parse_quote!(crate::data_structure::refbased),
    };
    let lsfg = LangSpecGen {
        bak: &lsf,
        sort2rs_type: |_, _| syn::parse_quote!(syn::Type),
        type_base_path: syn::parse_quote!(crate::types),
    };
    let m = gen(&bps, lsfg);
    let extension_of = extensionof_gen::gen(&bps.extension_of, &lsf);
    let data_structure =
        langdatastructure_gen::refbased::gen(&syn::parse_quote!(crate::data_structure), &lsf, true);
    prettyplease::unparse(&syn::parse_quote!(
        #m
        #extension_of
        pub mod data_structure {
            #data_structure
        }
    ))
}
