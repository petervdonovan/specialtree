use langspec::langspec::LangSpec;
use syn::parse_quote;

pub fn gen<L: LangSpec>(ls: &L) -> syn::TraitItem {
    parse_quote!(
        pub trait ExtensionOf {
            fn extension_of(&self) -> &str;
        }
    )
}
