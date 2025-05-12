use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{Name, TerminalLangSpec, ToLiteral},
    tymetafunc::{IdentifiedBy, RustTyMap, TyMetaFuncSpec},
};
use serde::{Deserialize, Serialize};
use term::Heaped;

pub fn parse_error() -> LangSpecFlat<ParseErrorTmfs> {
    let lsh: LangSpecHuman<ParseErrorTmfs> = serde_json::from_str(
        r#"
    {
        "name": {
            "human": "ParseError",
            "camel": "ParseError",
            "snake": "parse_error"
        },
        "products": [],
        "sums": []
    }
    "#,
    )
    .unwrap();
    LangSpecFlat::canonical_from(&lsh)
}

pub struct ParseErrorTmfs;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct ParseErrorTmfId();

impl ToLiteral for ParseErrorTmfId {
    fn to_literal(&self) -> syn::Expr {
        syn::parse_quote! {
            std_parse_error::ParseErrorTmfId()
        }
    }
}

impl TyMetaFuncSpec for ParseErrorTmfs {
    type TyMetaFuncId = ParseErrorTmfId;

    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> langspec::tymetafunc::TyMetaFuncData {
        match id {
            ParseErrorTmfId() => langspec::tymetafunc::TyMetaFuncData {
                name: Name {
                    human: "ParseError".into(),
                    camel: "ParseError".into(),
                    snake: "parse_error".into(),
                },
                args: Box::new([]),
                imp: RustTyMap {
                    ty_func: syn::parse_quote! {std_parse_error::ParseError},
                },
                idby: IdentifiedBy::Tmf,
                transparency: langspec::tymetafunc::Transparency::Transparent,
                heapbak: RustTyMap {
                    ty_func: syn::parse_quote! {std_parse_error::ParseErrorBak},
                },
                canonical_froms: Box::new([]),
                size_depends_on: Box::new([]),
            },
        }
    }

    fn my_type() -> syn::Type {
        syn::parse_quote! {
            std_parse_error::ParseErrorTmfs
        }
    }
}

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
#[derivative(Copy(bound = ""))]
pub struct ParseError<Heap>(pub parse::ParseError, std::marker::PhantomData<Heap>);
#[derive(Default)]
pub struct ParseErrorBak<Heap: ?Sized>(std::marker::PhantomData<Heap>);
impl<Heap> Heaped for ParseError<Heap> {
    type Heap = Heap;
}
impl<Heap> ParseError<Heap> {
    pub fn new(error: parse::ParseError) -> Self {
        Self(error, std::marker::PhantomData)
    }
}
