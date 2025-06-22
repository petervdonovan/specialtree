#![feature(let_chains)]

use ccf::DirectlyCanonicallyConstructibleFrom;
use derivative::Derivative;
use langspec::{
    langspec::Name,
    tymetafunc::{ArgId, RustTyMap, TyMetaFuncData, TyMetaFuncSpec},
};
use pmsp::Visitation;
use serde::{Deserialize, Serialize};
use term::{SuperHeap, TyMetaFunc};
use words::{Adtishness, NotAdtLike};

pub mod parse;
pub mod pattern_dyn;
pub mod unparse;

pub struct PatternTmfs;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PatternTmfsId {
    OrVariable,
    OrVariableZeroOrMore,
    NamedPattern,
}

impl TyMetaFuncSpec for PatternTmfs {
    type TyMetaFuncId = PatternTmfsId;

    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> TyMetaFuncData {
        let matched_ty_name = Name {
            human: "matched-ty".into(),
            camel: "MatchedTy".into(),
            snake: "matched_ty".into(),
        };
        match id {
            PatternTmfsId::OrVariable => TyMetaFuncData {
                name: Name {
                    human: "or-variable".into(),
                    camel: "OrVariable".into(),
                    snake: "or_variable".into(),
                },
                args: Box::new([matched_ty_name.clone()]),
                imp: RustTyMap {
                    ty_func: syn::parse_quote! { pattern_tmf::OrVariable },
                },
                heapbak: RustTyMap {
                    ty_func: syn::parse_quote! { pattern_tmf::OrVariableHeapBak },
                },
                idby: langspec::tymetafunc::IdentifiedBy::FirstTmfArg,
                canonical_froms: Box::new([Box::new([ArgId(0)])]),
                size_depends_on: Box::new([ArgId(0)]),
                is_collection_of: Box::new([]),
                transparency: langspec::tymetafunc::Transparency::Visible,
            },
            PatternTmfsId::OrVariableZeroOrMore => TyMetaFuncData {
                name: Name {
                    human: "or-variable-zero-or-more".into(),
                    camel: "OrVariableZeroOrMore".into(),
                    snake: "or_variable_zero_or_more".into(),
                },
                args: Box::new([matched_ty_name.clone()]),
                imp: RustTyMap {
                    ty_func: syn::parse_quote! { pattern_tmf::OrVariableZeroOrMore },
                },
                heapbak: RustTyMap {
                    ty_func: syn::parse_quote! { pattern_tmf::OrVariableZeroOrMoreHeapBak },
                },
                idby: langspec::tymetafunc::IdentifiedBy::FirstTmfArg,
                canonical_froms: Box::new([Box::new([ArgId(0)])]),
                size_depends_on: Box::new([ArgId(0)]),
                is_collection_of: Box::new([]),
                transparency: langspec::tymetafunc::Transparency::Visible,
            },
            PatternTmfsId::NamedPattern => TyMetaFuncData {
                name: Name {
                    human: "named-pattern".into(),
                    camel: "NamedPattern".into(),
                    snake: "named_pattern".into(),
                },
                args: Box::new([Name {
                    human: "pattern".into(),
                    camel: "Pattern".into(),
                    snake: "pattern".into(),
                }]),
                imp: RustTyMap {
                    ty_func: syn::parse_quote! { pattern_tmf::NamedPattern },
                },
                heapbak: RustTyMap {
                    ty_func: syn::parse_quote! { pattern_tmf::NamedPatternHeapBak },
                },
                idby: langspec::tymetafunc::IdentifiedBy::FirstTmfArg,
                canonical_froms: Box::new([]),
                size_depends_on: Box::new([ArgId(0)]),
                is_collection_of: Box::new([]),
                transparency: langspec::tymetafunc::Transparency::Visible,
            },
        }
    }
}

type Symbol = <string_interner::DefaultBackend as string_interner::backend::Backend>::Symbol;
#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct OrVariableHeapBak<Heap, MatchedTy> {
    names: string_interner::DefaultStringInterner,
    phantom: std::marker::PhantomData<(Heap, MatchedTy)>,
}
#[derive(Derivative)]
#[derivative(Clone(bound = "MatchedTy: Clone"))]
#[derivative(Copy(bound = "MatchedTy: Copy"))]
pub enum OrVariable<Heap, MatchedTy> {
    Ctor(MatchedTy),
    Variable { name: Symbol },
    Ignored(std::marker::PhantomData<Heap>),
}
impl<Heap, MatchedTy> TyMetaFunc for OrVariable<Heap, MatchedTy> {
    type HeapBak = OrVariableHeapBak<Heap, MatchedTy>;
}

#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct OrVariableZeroOrMoreHeapBak<Heap, MatchedTy> {
    names: string_interner::DefaultStringInterner,
    phantom: std::marker::PhantomData<(Heap, MatchedTy)>,
}
#[derive(Derivative)]
#[derivative(Clone(bound = "MatchedTy: Clone"))]
#[derivative(Copy(bound = "MatchedTy: Copy"))]
pub enum OrVariableZeroOrMore<Heap, MatchedTy> {
    Ctor(MatchedTy),
    Variable { name: Symbol },
    Ignored(std::marker::PhantomData<Heap>),
    ZeroOrMore { name: Symbol },
}
impl<Heap, MatchedTy> TyMetaFunc for OrVariableZeroOrMore<Heap, MatchedTy> {
    type HeapBak = OrVariableZeroOrMoreHeapBak<Heap, MatchedTy>;
}
#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct NamedPatternHeapBak<Heap, Pattern> {
    names: string_interner::DefaultStringInterner,
    phantom: std::marker::PhantomData<(Heap, Pattern)>,
}
#[derive(Derivative)]
#[derivative(Clone(bound = "Pattern: Clone"))]
#[derivative(Copy(bound = "Pattern: Copy"))]
pub struct NamedPattern<Heap, Pattern> {
    pub pattern: Pattern,
    name: Symbol,
    phantom: std::marker::PhantomData<Heap>,
}
impl<Heap, Pattern> TyMetaFunc for NamedPattern<Heap, Pattern> {
    type HeapBak = NamedPatternHeapBak<Heap, Pattern>;
}

impl<Heap, Pattern> NamedPattern<Heap, Pattern>
where
    Heap: SuperHeap<NamedPatternHeapBak<Heap, Pattern>>,
{
    pub fn name(&self, heap: &Heap) -> String {
        let subheap = heap.subheap::<NamedPatternHeapBak<_, _>>();
        subheap.names.resolve(self.name).unwrap().to_string()
    }
}

impl<Heap, MatchedTy> DirectlyCanonicallyConstructibleFrom<Heap, (MatchedTy, ())>
    for OrVariable<Heap, MatchedTy>
{
    fn construct(_heap: &mut Heap, t: (MatchedTy, ())) -> Self {
        OrVariable::Ctor(t.0)
    }

    fn deconstruct_succeeds(&self, _heap: &Heap) -> bool {
        matches!(self, OrVariable::Ctor(_))
    }

    fn deconstruct(self, _heap: &Heap) -> (MatchedTy, ()) {
        if let OrVariable::Ctor(ret) = self {
            (ret, ())
        } else {
            panic!()
        }
    }
}

impl<Heap, MatchedTy> DirectlyCanonicallyConstructibleFrom<Heap, (MatchedTy, ())>
    for OrVariableZeroOrMore<Heap, MatchedTy>
{
    fn construct(_heap: &mut Heap, t: (MatchedTy, ())) -> Self {
        OrVariableZeroOrMore::Ctor(t.0)
    }

    fn deconstruct_succeeds(&self, _heap: &Heap) -> bool {
        matches!(self, OrVariableZeroOrMore::Ctor(_))
    }

    fn deconstruct(self, _heap: &Heap) -> (MatchedTy, ()) {
        if let OrVariableZeroOrMore::Ctor(ret) = self {
            (ret, ())
        } else {
            panic!()
        }
    }
}

impl<Elem> Adtishness<Visitation> for OrVariable<(), Elem> {
    type X = NotAdtLike;
}
impl<Elem> Adtishness<Visitation> for OrVariableZeroOrMore<(), Elem> {
    type X = NotAdtLike;
}
impl<Elem> Adtishness<Visitation> for NamedPattern<(), Elem> {
    type X = NotAdtLike;
}
