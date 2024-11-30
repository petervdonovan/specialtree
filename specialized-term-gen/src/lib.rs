use langspec::langspec::LangSpec;
use langspec_gen_util::LangSpecGen;
use syn::parse_quote;

pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &L) -> syn::ItemMod {
    fn fail(_: &syn::Path, _: langspec::langspec::SortId<syn::Type>) -> syn::Type {
        panic!("must be type-agnostic");
    }
    let lg = LangSpecGen {
        bak: ls,
        sort2rs_type: fail,
        type_base_path: base_path.clone(),
    };
    let owned = owned::generate(base_path, &lg);
    let reference = reference::generate(base_path, &lg);
    let mut_reference = mut_reference::generate(base_path, &lg);
    let limpl_trait = limpl_trait(base_path, &lg);
    let byline = langspec_gen_util::byline!();
    parse_quote!(
        #byline
        pub mod extension_of {
            #limpl_trait
            #owned
            #reference
            #mut_reference
        }
    )
}
pub(crate) fn limpl_trait<L: LangSpec>(
    base_path: &syn::Path,
    ls: &LangSpecGen<L>,
) -> syn::ItemTrait {
    todo!()
}
mod owned {
    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::ItemMod {
        todo!()
    }
}
mod reference {
    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::ItemMod {
        todo!()
    }
}
mod mut_reference {
    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::ItemMod {
        todo!()
    }
}
