use langspec::{
    langspec::{Name, ToLiteral},
    tymetafunc::{IdentifiedBy, RustTyMap, TyMetaFuncData, TyMetaFuncSpec},
};
use serde::{Deserialize, Serialize};
use specialized_term::{CanonicallyConstructibleFrom, Heaped};

pub struct Core;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CoreTmfId(usize);

impl ToLiteral for CoreTmfId {
    fn to_literal(&self) -> syn::Expr {
        let lit = self.0;
        syn::parse_quote!(CoreTmfId(#lit))
    }
}

macro_rules! empty_heap_bak {
    ($name:ident $(,)? $($ty_args:ident),*) => {
        #[derive(Default)]
        pub struct $name<Heap, $($ty_args),*> {
            phantom: std::marker::PhantomData<(Heap, $($ty_args),*)>,
        }
    };
}

thread_local! {
    static CORE_BAK: once_cell::sync::Lazy<[TyMetaFuncData; 4]> =
    once_cell::sync::Lazy::new(|| {
        [TyMetaFuncData {
            name: Name {
                human: "nat-lit".into(),
                camel: "NatLit".into(),
                snake: "nat_lit".into(),
            },
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::BoundedNat),
            },
            args: vec![]
            .into_boxed_slice(),
            idby: IdentifiedBy::Tmf,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::BoundedNatHeapBak),
            },
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
            },
            args: vec![
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]
            .into_boxed_slice(),
            idby: IdentifiedBy::Tmf,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SetHeapBak),
            },
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
            },
            args: vec![
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]
            .into_boxed_slice(),
            idby: IdentifiedBy::Tmf,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SeqHeapBak),
            },
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        },
        TyMetaFuncData {
            name: Name {
                human: "idx_box".into(),
                camel: "IdxBox".into(),
                snake: "idx_box".into(),
            },
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::IdxBox),
            },
            args: vec![
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]
            .into_boxed_slice(),
            idby: IdentifiedBy::FirstTmfArg,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::IdxBoxHeapBak),
            },
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        }]
    });
}

impl TyMetaFuncSpec for Core {
    type TyMetaFuncId = CoreTmfId;

    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> TyMetaFuncData {
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
empty_heap_bak!(BoundedNatHeapBak);

pub struct Set<Heap, Elem> {
    heap: std::marker::PhantomData<Heap>,
    pub items: std::vec::Vec<Elem>,
}
impl<Heap, Elem> Heaped for Set<Heap, Elem> {
    type Heap = Heap;
}
empty_heap_bak!(SetHeapBak, Elem);

pub struct Seq<Heap, Elem> {
    heap: std::marker::PhantomData<Heap>,
    pub items: std::vec::Vec<Elem>,
}
impl<Heap, Elem> Heaped for Seq<Heap, Elem> {
    type Heap = Heap;
}
empty_heap_bak!(SeqHeapBak, Elem);

pub struct IdxBox<Heap, Elem> {
    phantom: std::marker::PhantomData<(Heap, Elem)>,
    pub idx: u32,
}
impl<Heap, Elem> Heaped for IdxBox<Heap, Elem> {
    type Heap = Heap;
}
impl<Heap, Elem> CanonicallyConstructibleFrom<Elem> for IdxBox<Heap, Elem> {
    fn construct(heap: &mut Self::Heap, t: Elem) -> Self {
        todo!()
    }

    fn deconstruct(self) -> Elem {
        todo!()
    }
}
#[derive(Default)]
pub struct IdxBoxHeapBak<Heap, Elem> {
    phantom: std::marker::PhantomData<(Heap, Elem)>,
    elems: std::vec::Vec<Elem>,
}
