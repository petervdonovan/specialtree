use langspec::{
    langspec::{Name, ToLiteral},
    tymetafunc::{RustTyMap, TyMetaFuncData, TyMetaFuncSpec},
};
use serde::{Deserialize, Serialize};
use specialized_term::Heaped;

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
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::BoundedNat),
                args: vec![].into_boxed_slice(),
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
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::Set),
                args: vec![CoreTmfArgId(0)].into_boxed_slice(),
            },
            args: vec![
                (CoreTmfArgId(0), Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }),
            ]
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
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::Seq),
                args: vec![CoreTmfArgId(0)].into_boxed_slice(),
            },
            args: vec![
                (CoreTmfArgId(0), Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }),
            ]
            .into_boxed_slice(),
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        }]
    });
}

impl TyMetaFuncSpec for Core {
    type TyMetaFuncId = CoreTmfId;
    type TyMetaFuncArgId = CoreTmfArgId;

    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> TyMetaFuncData<Self::TyMetaFuncArgId> {
        CORE_BAK.with(|cb| cb[id.0].clone())
    }
}
pub struct BoundedNat<Heap> {
    heap: std::marker::PhantomData<Heap>,
    pub n: usize,
}
impl<Heap> Heaped for BoundedNat<Heap> {
    type Heap = Heap;
}

pub struct Set<Heap, Elem> {
    heap: std::marker::PhantomData<Heap>,
    pub items: std::vec::Vec<Elem>,
}
impl<Heap, Elem> Heaped for Set<Heap, Elem> {
    type Heap = Heap;
}

pub struct Seq<Heap, Elem> {
    heap: std::marker::PhantomData<Heap>,
    pub items: std::vec::Vec<Elem>,
}
