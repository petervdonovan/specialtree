use langspec::{
    langspec::{Name, ToLiteral},
    tymetafunc::{TyMetaFuncData, TyMetaFuncSpec},
};
use serde::{Deserialize, Serialize};

pub struct Core;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CoreTmfId(usize);
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CoreTmfArgId(usize);

impl ToLiteral for CoreTmfId {
    fn to_literal(&self) -> syn::Expr {
        let lit = self.0;
        syn::parse_quote!(CoreTmfId(#lit))
    }
}

impl ToLiteral for CoreTmfArgId {
    fn to_literal(&self) -> syn::Expr {
        let lit = self.0;
        syn::parse_quote!(CoreTmfArgId(#lit))
    }
}

thread_local! {
    static CORE_BAK: once_cell::sync::Lazy<[TyMetaFuncData<CoreTmfArgId>; 3]> =
    once_cell::sync::Lazy::new(|| {
        [TyMetaFuncData {
            name: Name {
                human: "nat-lit".into(),
                camel: "NatLit".into(),
                snake: "nat_lit".into(),
            },
            args: vec![]
            .into_boxed_slice(),
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        },
        TyMetaFuncData {
            name: Name {
                human: "set".into(),
                camel: "Set".into(),
                snake: "set".into(),
            },
            args: vec![]
            .into_boxed_slice(),
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        },
        TyMetaFuncData {
            name: Name {
                human: "seq".into(),
                camel: "Seq".into(),
                snake: "seq".into(),
            },
            args: vec![]
            .into_boxed_slice(),
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        }]
    });
}

impl TyMetaFuncSpec for Core {
    type TyMetaFuncId = CoreTmfId;
    type TyMetaFuncArgId = CoreTmfArgId;

    fn ty_meta_funcs(&self) -> impl Iterator<Item = Self::TyMetaFuncId> {
        (0..CORE_BAK.with(|cb| cb.len())).map(CoreTmfId)
    }

    fn ty_meta_func_data(&self, id: &Self::TyMetaFuncId) -> TyMetaFuncData<Self::TyMetaFuncArgId> {
        CORE_BAK.with(|cb| cb[id.0].clone())
    }
}
