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
    lg: &LangSpecGen<L>,
) -> syn::ItemFn {
    let byline = langspec_gen_util::byline!();
    let camel_ident = lg.common_gen_datas().camel_ident;
    syn::parse_quote! {
        #byline
        fn parse<LImpl: #extension_of::LImpl>(input: &str) -> Result<(LImpl, #extension_of::Any<LImpl>), ()> {
            #(
                let refbased: Result<#refbased_data_structure::#camel_ident, _> = serde_yml::from_str(input);
                if let Ok(refbased) = refbased {
                    use #extension_of::reference::#camel_ident;
                    let l = core::default::Default::default();
                    let mut lo = core::default::Default::default();
                    let ret = refbased.convert(&l, &mut lo);
                    return Ok((lo, #extension_of::Any::#camel_ident(ret)));
                }
            )*
            Err(())
        }
    }
}

pub fn formatted(lsh: &LangSpecHuman) -> String {
    let lsf: LangSpecFlat = LangSpecFlat::canonical_from(lsh);
    let bps = BasePaths {
        extension_of: syn::parse_quote!(crate::extension_of),
        refbased_data_structure: syn::parse_quote!(crate::data_structure::refbased),
    };
    let idxbased_data_structure_path: syn::Path =
        syn::parse_quote!(crate::data_structure::idxbased);
    // let idxbased_extensionof_path: syn::Path = syn::parse_quote!(crate::extension_of::idxbased);
    let lsfg = LangSpecGen {
        bak: &lsf,
        sort2rs_type: |_, _| syn::parse_quote!(syn::Type),
        type_base_path: syn::parse_quote!(crate::types),
    };
    let m = gen(&bps, &lsfg);
    let refbased_extension_of = refbased_extensionof_gen::gen(
        &refbased_extensionof_gen::BasePaths {
            extension_of: bps.extension_of.clone(),
            data_structure: bps.refbased_data_structure.clone(),
        },
        &lsf,
    );
    let idxbased_extension_of = idxbased_extensionof_gen::gen(
        &idxbased_extensionof_gen::BasePaths {
            extension_of: bps.extension_of.clone(),
            data_structure: idxbased_data_structure_path.clone(),
        },
        &lsf,
    );
    let extension_of = extensionof_gen::gen(&bps.extension_of, &lsf);
    let data_structure =
        langdatastructure_gen::refbased::gen(&syn::parse_quote!(crate::data_structure), &lsf, true);
    let idxbased_data_structure =
        langdatastructure_gen::idxbased::gen(&syn::parse_quote!(crate::data_structure), &lsf);
    let ls_camel_name = lsfg.camel_ident();
    let test: syn::ItemFn = syn::parse_quote! {
        #[test]
        fn round_trip() {
            let input = std::fs::read_to_string("tests/fib.yml").unwrap();
            let (l, lo) = parse::<#idxbased_data_structure_path::#ls_camel_name>(&input).unwrap();
            let expected = expect_test::expect![[]];
            expected.assert_debug_eq(&lo);
        }
    };
    prettyplease::unparse(&syn::parse_quote!(
        #m
        #refbased_extension_of
        #idxbased_extension_of
        #extension_of
        pub mod data_structure {
            #data_structure
            #idxbased_data_structure
        }
        #test
    ))
}
