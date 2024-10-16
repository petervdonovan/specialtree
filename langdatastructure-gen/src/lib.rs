use langspec::humanreadable::LangSpecHuman;
use syn::{ItemEnum, ItemStruct};

pub fn data_structure(lsh: LangSpecHuman) -> (Vec<ItemStruct>, Vec<ItemEnum>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_structure() {
        let lsh = langspec_examples::fib();
        let (structs, enums) = data_structure(lsh);
        let expected = expect_test::expect![[r#""#]];
        expected.assert_eq(
            &quote::quote! {
                #(#structs)*
                #(#enums)*
            }
            .to_string(),
        );
    }
}
