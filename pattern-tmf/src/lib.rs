use ccf::DirectlyCanonicallyConstructibleFrom;
use derivative::Derivative;
use langspec::{
    langspec::Name,
    tymetafunc::{ArgId, RustTyMap, TyMetaFuncData, TyMetaFuncSpec},
};
use serde::{Deserialize, Serialize};

pub mod parse;

pub struct PatternTmfs;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PatternTmfsId {
    // Variable,
    // ZeroOrMore,
    // Ignored,
    OrVariable,
    OrVariableZeroOrMore,
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
            // PatternTmfsId::Variable => TyMetaFuncData {
            //     name: Name {
            //         human: "variable".into(),
            //         camel: "Variable".into(),
            //         snake: "variable".into(),
            //     },
            //     args: Box::new([matched_ty_name.clone()]),
            //     imp: RustTyMap {
            //         ty_func: syn::parse_quote! { pattern_tmf::Variable },
            //     },
            //     idby: langspec::tymetafunc::IdentifiedBy::Tmf,
            //     heapbak: RustTyMap {
            //         ty_func: syn::parse_quote! { pattern_tmf::VariableHeapBak },
            //     },
            //     canonical_froms: Box::new([]),
            //     size_depends_on: Box::new([]),
            //     is_collection_of: Box::new([]),
            //     transparency: langspec::tymetafunc::Transparency::Visible,
            // },
            // PatternTmfsId::ZeroOrMore => TyMetaFuncData {
            //     name: Name {
            //         human: "...".into(),
            //         camel: "ZeroOrMore".into(),
            //         snake: "zero_or_more".into(),
            //     },
            //     args: Box::new([matched_ty_name.clone()]),
            //     imp: RustTyMap {
            //         ty_func: syn::parse_quote! { pattern_tmf::ZeroOrMore },
            //     },
            //     idby: langspec::tymetafunc::IdentifiedBy::Tmf,
            //     heapbak: RustTyMap {
            //         ty_func: syn::parse_quote! { pattern_tmf::ZeroOrMoreHeapBak },
            //     },
            //     canonical_froms: Box::new([]),
            //     size_depends_on: Box::new([]),
            //     is_collection_of: Box::new([]),
            //     transparency: langspec::tymetafunc::Transparency::Visible,
            // },
            // PatternTmfsId::Ignored => TyMetaFuncData {
            //     name: Name {
            //         human: "_".into(),
            //         camel: "Ignored".into(),
            //         snake: "ignored".into(),
            //     },
            //     args: Box::new([matched_ty_name.clone()]),
            //     imp: RustTyMap {
            //         ty_func: syn::parse_quote! { pattern_tmf::Ignored },
            //     },
            //     idby: langspec::tymetafunc::IdentifiedBy::Tmf,
            //     heapbak: RustTyMap {
            //         ty_func: syn::parse_quote! { pattern_tmf::IgnoredHeapBak },
            //     },
            //     canonical_froms: Box::new([]),
            //     size_depends_on: Box::new([]),
            //     is_collection_of: Box::new([]),
            //     transparency: langspec::tymetafunc::Transparency::Visible,
            // },
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
                idby: langspec::tymetafunc::IdentifiedBy::FirstTmfArg,
                heapbak: RustTyMap {
                    ty_func: syn::parse_quote! { pattern_tmf::OrVariableHeapBak },
                },
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
                idby: langspec::tymetafunc::IdentifiedBy::FirstTmfArg,
                heapbak: RustTyMap {
                    ty_func: syn::parse_quote! { pattern_tmf::OrVariableZeroOrMoreHeapBak },
                },
                canonical_froms: Box::new([Box::new([ArgId(0)])]),
                size_depends_on: Box::new([ArgId(0)]),
                is_collection_of: Box::new([]),
                transparency: langspec::tymetafunc::Transparency::Visible,
            },
        }
    }
}

// pub struct Variable<MatchedTy> {
//     pub name: String,
//     phantom: std::marker::PhantomData<MatchedTy>,
// }
// empty_heap_bak!(VariableHeapBak);
// pub struct ZeroOrMore<MatchedTy> {
//     pub name: String,
//     phantom: std::marker::PhantomData<MatchedTy>,
// }
// empty_heap_bak!(ZeroOrMoreHeapBak);
// pub struct Ignored<MatchedTy> {
//     phantom: std::marker::PhantomData<MatchedTy>,
// }
// empty_heap_bak!(IgnoredHeapBak);

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
