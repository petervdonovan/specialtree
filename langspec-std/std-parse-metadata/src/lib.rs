use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{Name, TerminalLangSpec, ToLiteral},
    tymetafunc::{IdentifiedBy, RustTyMap, TyMetaFuncSpec},
};
use serde::{Deserialize, Serialize};
use term::Heaped;

pub fn parse_metadata() -> LangSpecFlat<ParseMetadataTmfs> {
    let lsh: LangSpecHuman<ParseMetadataTmfs> = serde_json::from_str(
        r#"
    {
        "name": {
            "human": "ParseMetadata",
            "camel": "ParseMetadata",
            "snake": "parse_metadata"
        },
        "products": [],
        "sums": []
    }
    "#,
    )
    .unwrap();
    LangSpecFlat::canonical_from(&lsh)
}

pub struct ParseMetadataTmfs;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ParseMetadataTmfId();

impl ToLiteral for ParseMetadataTmfId {
    fn to_literal(&self) -> syn::Expr {
        syn::parse_quote! {
            std_parse_metadata::ParseMetadataTmfId()
        }
    }
}

impl TyMetaFuncSpec for ParseMetadataTmfs {
    type TyMetaFuncId = ParseMetadataTmfId;

    type OpaqueTerm = parse::ParseMetadata;

    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> langspec::tymetafunc::TyMetaFuncData {
        match id {
            ParseMetadataTmfId() => langspec::tymetafunc::TyMetaFuncData {
                name: Name {
                    human: "ParseMetadata".into(),
                    camel: "ParseMetadata".into(),
                    snake: "parse_metadata".into(),
                },
                args: Box::new([]),
                imp: RustTyMap {
                    ty_func: syn::parse_quote! {std_parse_metadata::ParseMetadata},
                },
                idby: IdentifiedBy::Tmf,
                transparency: langspec::tymetafunc::Transparency::Visible,
                heapbak: RustTyMap {
                    ty_func: syn::parse_quote! {std_parse_metadata::ParseMetadataBak},
                },
                // maybe_conversions: Box::new([]),
                canonical_froms: Box::new([]),
            },
        }
    }

    fn my_type() -> syn::Type {
        syn::parse_quote! {
            std_parse_metadata::ParseMetadataTmfs
        }
    }
}

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
#[derivative(Copy(bound = ""))]
pub struct ParseMetadata<Heap>(pub parse::ParseMetadata, std::marker::PhantomData<Heap>);
#[derive(Default)]
pub struct ParseMetadataBak<Heap: ?Sized>(std::marker::PhantomData<Heap>);
impl<Heap> Heaped for ParseMetadata<Heap> {
    type Heap = Heap;
}
impl<Heap> ParseMetadata<Heap> {
    pub fn new(parse_metadata: parse::ParseMetadata) -> Self {
        Self(parse_metadata, std::marker::PhantomData)
    }
}
