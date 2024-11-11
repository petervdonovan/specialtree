use langspec::{flat::LangSpecFlat, humanreadable::LangSpecHuman, langspec::TerminalLangSpec as _};
use langspec_gen_util::LangSpecGen;

pub struct BasePaths {
    pub refbased_data_structure: syn::Path,
    pub extension_of: syn::Path,
}

pub fn gen(
    BasePaths {
        refbased_data_structure,
        extension_of,
    }: &BasePaths,
    ty_camel_ident: &syn::Ident,
) -> (syn::ItemFn, syn::ItemFn) {
    let byline = langspec_gen_util::byline!();
    let parse = syn::parse_quote! {
        #byline
        fn parse<LImpl: #extension_of::LImpl, #ty_camel_ident: #extension_of::owned::#ty_camel_ident<LImpl = LImpl>>(input: &str) -> Result<(LImpl, #ty_camel_ident), ()> {
            let refbased: #refbased_data_structure::#ty_camel_ident = serde_json::from_str(input).map_err(|e| println!("Parse error: {e}"))?;
            use #extension_of::reference::#ty_camel_ident;
            let l = core::default::Default::default();
            let mut lo = core::default::Default::default();
            let ret = refbased.convert(&l, &mut lo);
            Ok((lo, ret))
        }
    };
    let unparse = syn::parse_quote! {
        #byline
        fn unparse<
            LImpl: #extension_of::LImpl,
            #ty_camel_ident: #extension_of::owned::#ty_camel_ident<LImpl = LImpl>
        >(
            lo: &LImpl, own: #ty_camel_ident
        ) -> Result<String, ()> {
            use #extension_of::reference::#ty_camel_ident;
            let mut l = core::default::Default::default();
            let refbased: #refbased_data_structure::#ty_camel_ident = own.get_ref(lo).convert(lo, &mut l);
            serde_json::to_string_pretty(&refbased).map_err(|_| ())
        }
    };
    (parse, unparse)
}

pub fn formatted(lsh: &LangSpecHuman) -> String {
    let lsf: LangSpecFlat = LangSpecFlat::canonical_from(lsh);
    let bps = BasePaths {
        extension_of: syn::parse_quote!(crate::extension_of),
        refbased_data_structure: syn::parse_quote!(crate::data_structure::refbased),
    };
    let idxbased_data_structure_path: syn::Path =
        syn::parse_quote!(crate::data_structure::idxbased);
    let lsfg = LangSpecGen {
        bak: &lsf,
        sort2rs_type: |_, _| syn::parse_quote!(syn::Type),
        type_base_path: syn::parse_quote!(crate::types),
    };
    let (parse, unparse) = gen(&bps, &syn::parse_quote! {Nat});
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
            let input = std::fs::read_to_string("tests/fib.json").unwrap();
            let (lo, nat) = parse::<#idxbased_data_structure_path::#ls_camel_name, #idxbased_data_structure_path::Nat>(&input).unwrap();
            let unparsed = unparse(&lo, nat).unwrap();
            let expected = expect_test::expect![[r#"
{
  "Plus": [
    {
      "F": {
        "NatLit": 50
      }
    },
    {
      "F": {
        "NatLit": 51
      }
    }
  ]
}"#]];
            expected.assert_eq(&unparsed);
        }
    };
    prettyplease::unparse(&syn::parse_quote!(
        #parse
        #unparse
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
