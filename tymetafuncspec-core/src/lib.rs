use derivative::Derivative;
use langspec::{
    langspec::{Name, ToLiteral},
    tymetafunc::{IdentifiedBy, RustTyMap, Transparency, TyMetaFuncData, TyMetaFuncSpec},
};
use parse::{miette::SourceSpan, Parse, Unparse};
use serde::{Deserialize, Serialize};
use term::{CanonicallyConstructibleFrom, Heaped, SuperHeap, TyFingerprint, UnsafeHeapDrop};
use term_unspecialized::{FromTermError, MaybeOpaqueTerm, Term, TermRoundTrip};

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
        #[derive(derivative::Derivative)]
        #[derivative(Default(bound=""))]
        pub struct $name<Heap: ?Sized, $($ty_args),*> {
            phantom: std::marker::PhantomData<($($ty_args,)* Heap,)>,
        }
    };
}

pub const NATLIT: CoreTmfId = CoreTmfId(0);
pub const SET: CoreTmfId = CoreTmfId(1);
pub const SEQ: CoreTmfId = CoreTmfId(2);
pub const IDXBOX: CoreTmfId = CoreTmfId(3);
pub const EITHER: CoreTmfId = CoreTmfId(4);
pub const MAYBE: CoreTmfId = CoreTmfId(5);
pub const PAIR: CoreTmfId = CoreTmfId(6);

thread_local! {
    static CORE_BAK: once_cell::sync::Lazy<[TyMetaFuncData; 7]> =
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
            args: Box::new([]),
            idby: IdentifiedBy::Tmf,
            transparency: Transparency::Visible,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::BoundedNatHeapBak),
            },
            maybe_conversions: Box::new([]),
            canonical_froms: Box::new([]),
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
            args: Box::new([
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]),
            idby: IdentifiedBy::Tmf,
            transparency: Transparency::Visible,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SetHeapBak),
            },
            maybe_conversions: Box::new([]),
            canonical_froms: Box::new([]),
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
            args: Box::new([
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]),
            idby: IdentifiedBy::Tmf,
            transparency: Transparency::Visible,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SeqHeapBak),
            },
            maybe_conversions: Box::new([]),
            canonical_froms: Box::new([]),
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
            transparency: Transparency::Visible,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::IdxBoxHeapBak),
            },
            maybe_conversions: Box::new([]),
            canonical_froms: Box::new([]),
        },
        TyMetaFuncData {
            name: Name {
                human: "either".into(),
                camel: "Either".into(),
                snake: "either".into()
            },
            args: Box::new([
                Name {human: "l".into(), camel: "L".into(), snake: "l".into()},
                Name {human: "r".into(), camel: "R".into(), snake: "r".into()},
            ]),
            imp: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::Either) },
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Transparent,
            heapbak: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::EitherHeapBak) },
            maybe_conversions: Box::new([]),
            canonical_froms: Box::new([])
        },
        TyMetaFuncData {
            name: Name {
                human: "maybe".into(),
                camel: "Maybe".into(),
                snake: "maybe".into()
            },
            args: Box::new([
                Name {human: "t".into(), camel: "T".into(), snake: "t".into()},
            ]),
            imp: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::Maybe) },
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Transparent,
            heapbak: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::MaybeHeapBak) },
            maybe_conversions: Box::new([]),
            canonical_froms: Box::new([])
        },
        TyMetaFuncData {
            name: Name {
                human: "pair".into(),
                camel: "Pair".into(),
                snake: "pair".into()
            },
            args: Box::new([
                Name {human: "l".into(), camel: "L".into(), snake: "l".into()},
                Name {human: "r".into(), camel: "R".into(), snake: "r".into()},
            ]),
            imp: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::Pair) },
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Visible,
            heapbak: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::PairHeapBak) },
            maybe_conversions: Box::new([]),
            canonical_froms: Box::new([]),
        }
        ]
    });
}

impl TyMetaFuncSpec for Core {
    type TyMetaFuncId = CoreTmfId;
    type OpaqueTerm = usize;

    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> TyMetaFuncData {
        CORE_BAK.with(|cb| cb[id.0].clone())
    }

    fn my_type() -> syn::Type {
        syn::parse_quote!(tymetafuncspec_core::Core)
    }
}
pub type Cstfy<Heap, T> =
    CstfyTransparent<Heap, Pair<Heap, T, Maybe<Heap, std_parse_metadata::ParseMetadata<Heap>>>>;
pub type CstfyTransparent<Heap, T> = Either<Heap, T, std_parse_error::ParseError<Heap>>;
pub fn cstfy_ok<Heap, T>(
    t: T,
    starting_offset: parse::miette::SourceOffset,
    ending_offset: parse::miette::SourceOffset,
) -> Cstfy<Heap, T> {
    Either::Left(
        Pair {
            l: t,
            r: Maybe::Just(
                std_parse_metadata::ParseMetadata::new(parse::ParseMetadata {
                    location: SourceSpan::new(
                        starting_offset,
                        ending_offset.offset() - starting_offset.offset(),
                    ),
                }),
                std::marker::PhantomData,
            ),
            heap: std::marker::PhantomData,
        },
        std::marker::PhantomData,
    )
}
pub fn cstfy_transparent_ok<Heap, T>(t: T) -> CstfyTransparent<Heap, T> {
    Either::Left(t, std::marker::PhantomData)
}
pub fn uncstfy<Heap, T>(c: &Cstfy<Heap, T>) -> Option<&T> {
    match c {
        Either::Left(p, _) => Some(&p.l),
        Either::Right(_, _) => None,
    }
}
pub fn uncstfy_transparent<Heap, T>(c: &CstfyTransparent<Heap, T>) -> Option<&T> {
    match c {
        Either::Left(t, _) => Some(t),
        Either::Right(_, _) => None,
    }
}
macro_rules! return_if_err {
    ($result:ident, $offset:expr) => {
        let $result = match $result {
            Ok(ok) => ok,
            Err(e) => {
                return (
                    Either::Right(
                        std_parse_error::ParseError::new(e),
                        std::marker::PhantomData,
                    ),
                    $offset,
                )
            }
        };
    };
}
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""))]
#[derivative(Clone(bound = ""))]
#[derivative(Copy(bound = ""))]
pub struct BoundedNat<Heap> {
    heap: std::marker::PhantomData<Heap>,
    pub n: usize,
}
impl<Heap> Heaped for BoundedNat<Heap> {
    type Heap = Heap;
}
impl<Heap> UnsafeHeapDrop<Heap> for BoundedNat<Heap> {
    unsafe fn unsafe_heap_drop(self, _: &mut Heap) {}
}
empty_heap_bak!(BoundedNatHeapBak);
impl<Heap: SuperHeap<BoundedNatHeapBak<Heap>>> TermRoundTrip<0, Core> for BoundedNat<Heap> {
    fn to_term(self, _: &mut Heap) -> Term<0, Core, Heap> {
        let fingerprint = TyFingerprint::from(stringify!(tymetafuncspec_core::BoundedNat));
        Term::new(fingerprint, vec![MaybeOpaqueTerm::Opaque(self.n)])
    }

    fn from_term(_: &mut Heap, t: Term<0, Core, Heap>) -> Result<Self, FromTermError> {
        let n = match t.args[0] {
            MaybeOpaqueTerm::Opaque(n) => n,
            _ => Err(FromTermError::Invalid)?,
        };
        Ok(Self {
            heap: std::marker::PhantomData,
            n,
        })
    }
}
impl<Heap> parse::Parse<Heap> for Cstfy<Heap, BoundedNat<Heap>>
where
    Heap: SuperHeap<BoundedNatHeapBak<Heap>>,
{
    fn parse(
        source: &str,
        offset: parse::miette::SourceOffset,
        _heap: &mut Heap,
        _errors: &mut Vec<parse::ParseError>,
    ) -> (Self, parse::miette::SourceOffset) {
        let next_unicode_word = parse::unicode_segmentation::UnicodeSegmentation::unicode_words(
            &source[offset.offset()..],
        )
        .next()
        .ok_or(parse::ParseError::UnexpectedEndOfInput(offset.into()));
        return_if_err!(next_unicode_word, offset);
        let n = next_unicode_word
            .parse::<usize>()
            .map_err(|_| parse::ParseError::TmfsParseFailure(offset.into()));
        return_if_err!(n, offset);
        let new_offset =
            parse::miette::SourceOffset::from(offset.offset() + next_unicode_word.len());
        (
            cstfy_ok(
                BoundedNat {
                    heap: std::marker::PhantomData,
                    n,
                },
                offset,
                new_offset,
            ),
            new_offset,
        )
    }

    fn unparse(&self, _: &Self::Heap) -> parse::Unparse {
        uncstfy(self).map_or(Unparse::new("err"), |n| Unparse::new(&n.n.to_string()))
    }
}
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""))]
#[derivative(Clone(bound = ""))]
#[derivative(Copy(bound = ""))]
pub struct Set<Heap, Elem> {
    heap: std::marker::PhantomData<(Heap, Elem)>,
    pub items: usize,
}
impl<Heap, Elem: Heaped<Heap = Heap>> Heaped for Set<Heap, Elem> {
    type Heap = Heap;
}
#[derive(derivative::Derivative)]
#[derivative(Default(bound = ""))]
pub struct SetHeapBak<Heap: ?Sized, Elem> {
    phantom: std::marker::PhantomData<Heap>,
    vecs: slab::Slab<std::vec::Vec<Elem>>,
}
impl<Heap: SuperHeap<SetHeapBak<Heap, Elem>>, Elem> UnsafeHeapDrop<Heap> for Set<Heap, Elem> {
    unsafe fn unsafe_heap_drop(self, heap: &mut Heap) {
        let mut subheap = heap.subheap_mut::<SetHeapBak<Heap, Elem>>();
        todo!()
    }
}
// macro_rules! collection_term_round_trip_impl {
//     ($collection:ty, $globally_unique_name:path) => {
//         impl<Heap, Elem: TermRoundTrip<0, Core> + Heaped<Heap = Heap>> TermRoundTrip<0, Core>
//             for $collection
//         {
//             fn to_term(self, heap: &mut Self::Heap) -> Term<0, Core, Heap> {
//                 let items_terms: Vec<Term<0, Core, Heap>> = self
//                     .items
//                     .into_iter()
//                     .map(|item| item.to_term(heap))
//                     .collect();
//                 let mut fingerprint = TyFingerprint::from(stringify!($globally_unique_name));
//                 for item in &items_terms {
//                     fingerprint = fingerprint.combine(&item.tyfingerprint);
//                 }
//                 Term::new(
//                     fingerprint,
//                     items_terms.into_iter().map(MaybeOpaqueTerm::Term).collect(),
//                 )
//             }
//             fn from_term(
//                 heap: &mut Self::Heap,
//                 t: Term<0, Core, Heap>,
//             ) -> Result<Self, FromTermError> {
//                 let mut items = Vec::new();
//                 let mut fingerprint = TyFingerprint::from(stringify!($globally_unique_name));
//                 for item in t.args.into_iter().filter_map(|arg| match arg {
//                     MaybeOpaqueTerm::Term(t) => Some(t),
//                     _ => None,
//                 }) {
//                     fingerprint = fingerprint.combine(&item.tyfingerprint);
//                     items.push(Elem::from_term(heap, item)?);
//                 }
//                 if fingerprint != t.tyfingerprint {
//                     panic!("Dynamic cast failed");
//                 }
//                 Ok(Self {
//                     items,
//                     heap: std::marker::PhantomData,
//                 })
//             }
//         }
//     };
// }
// collection_term_round_trip_impl!(Set<Heap, Elem>, tymetafuncspec_core::Set);
impl<Heap, Elem> parse::Parse<Heap> for Cstfy<Heap, Set<Heap, Elem>>
where
    Heap: SuperHeap<SetHeapBak<Heap, Elem>>,
    Elem: parse::Parse<Heap>,
{
    fn parse(
        source: &str,
        initial_offset: parse::miette::SourceOffset,
        heap: &mut Self::Heap,
        errors: &mut Vec<parse::ParseError>,
    ) -> (
        Either<
            Heap,
            Pair<Heap, Set<Heap, Elem>, Maybe<Heap, std_parse_metadata::ParseMetadata<Heap>>>,
            std_parse_error::ParseError<Heap>,
        >,
        parse::miette::SourceOffset,
    ) {
        let mut items = Vec::new();
        let mut offset = initial_offset;
        while !source[offset.offset()..].starts_with('}') {
            let (item, new_offset) =
                <Elem as parse::Parse<Heap>>::parse(source, offset, heap, errors);
            items.push(item);
            offset = new_offset;
            if source[offset.offset()..].starts_with(',') {
                offset = parse::miette::SourceOffset::from(offset.offset() + 1);
            }
        }
        (
            cstfy_ok(
                Set {
                    items: todo!(),
                    heap: std::marker::PhantomData,
                },
                initial_offset,
                offset,
            ),
            offset,
        )
    }
    fn unparse(&self, heap: &Self::Heap) -> Unparse {
        uncstfy(self).map_or(Unparse::new("err"), |s| {
            let mut unparse = Unparse::new("{");
            unparse.indent();
            todo!();
            // for item in &s.items {
            //     unparse.vstack(item.unparse(heap));
            // }
            // unparse.hstack(Unparse::new("}"));
            unparse
        })
    }
}
#[derive(Debug)]
pub struct Seq<Heap, Elem> {
    heap: std::marker::PhantomData<Heap>,
    pub items: std::vec::Vec<Elem>,
}
impl<Heap, Elem> Heaped for Seq<Heap, Elem> {
    type Heap = Heap;
}
empty_heap_bak!(SeqHeapBak, Elem);
// collection_term_round_trip_impl!(Seq<Heap, Elem>, tymetafuncspec_core::Seq);
impl<Heap, T> parse::Parse<Heap> for Cstfy<Heap, Seq<Heap, T>>
where
    Heap: SuperHeap<SeqHeapBak<Heap, T>>,
    T: parse::Parse<Heap> + Heaped<Heap = Heap>,
{
    fn parse(
        source: &str,
        initial_offset: parse::miette::SourceOffset,
        heap: &mut Self::Heap,
        errors: &mut Vec<parse::ParseError>,
    ) -> (Cstfy<Heap, Seq<Heap, T>>, parse::miette::SourceOffset) {
        let mut items = Vec::new();
        let mut offset = initial_offset;
        while !source[offset.offset()..].starts_with(']') {
            let (item, new_offset) = <T as parse::Parse<Heap>>::parse(source, offset, heap, errors);
            items.push(item);
            offset = new_offset;
            if source[offset.offset()..].starts_with(',') {
                offset = parse::miette::SourceOffset::from(offset.offset() + 1);
            }
        }
        (
            cstfy_ok(
                Seq {
                    items,
                    heap: std::marker::PhantomData,
                },
                initial_offset,
                offset,
            ),
            offset,
        )
    }

    fn unparse(&self, heap: &Self::Heap) -> Unparse {
        uncstfy(self).map_or(Unparse::new("err"), |s| {
            let mut unparse = Unparse::new("[");
            unparse.indent();
            for item in &s.items {
                unparse.vstack(item.unparse(heap));
            }
            unparse.hstack(Unparse::new("]"));
            unparse
        })
    }
}
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""))]
#[derivative(Clone(bound = ""))]
#[derivative(Copy(bound = ""))]
pub struct IdxBox<Heap, Elem> {
    phantom: std::marker::PhantomData<(Heap, Elem)>,
    pub idx: u32,
}
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem> Heaped for IdxBox<Heap, Elem> {
    type Heap = Heap;
}
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem: Copy> UnsafeHeapDrop<Heap>
    for IdxBox<Heap, Elem>
{
    unsafe fn unsafe_heap_drop(self, heap: &mut Heap) {
        heap.subheap_mut::<IdxBoxHeapBak<Heap, Elem>>().elems[self.idx as usize];
    }
}
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem: Copy> CanonicallyConstructibleFrom<Elem>
    for IdxBox<Heap, Elem>
{
    fn construct(heap: &mut Self::Heap, t: Elem) -> Self {
        todo!()
    }

    fn deconstruct_succeeds(&self, heap: &Self::Heap) -> bool {
        todo!()
    }

    fn deconstruct(self, heap: &Self::Heap) -> Elem {
        todo!()
    }
}
// impl<
//         Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
//         Elem: TermRoundTrip<0, Core> + Heaped<Heap = Heap>,
//     > TermRoundTrip<0, Core> for IdxBox<Heap, Elem>
// {
//     fn to_term(self, heap: &mut Heap) -> Term<0, Core, Heap> {
//         let item_term = heap
//             .subheap_mut::<IdxBoxHeapBak<Heap, Elem>>()
//             .elems
//             .get_mut(self.idx as usize)
//             .unwrap()
//             .take()
//             .unwrap()
//             .to_term(heap);
//         let mut fingerprint = TyFingerprint::from(stringify!(tymetafuncspec_core::IdxBox));
//         fingerprint = fingerprint.combine(&item_term.tyfingerprint);
//         Term::new(fingerprint, vec![MaybeOpaqueTerm::Term(item_term)])
//     }

//     fn from_term(heap: &mut Heap, t: Term<0, Core, Heap>) -> Result<Self, FromTermError> {
//         let item = Elem::from_term(heap, t.args[0].clone().to_term())?;
//         let idx = <Heap as SuperHeap<IdxBoxHeapBak<Heap, Elem>>>::subheap::<
//             IdxBoxHeapBak<Heap, Elem>,
//         >(heap)
//         .elems
//         .len();
//         heap.subheap_mut::<IdxBoxHeapBak<Heap, Elem>>()
//             .elems
//             .push(Some(item));
//         Ok(Self {
//             phantom: std::marker::PhantomData,
//             idx: idx as u32,
//         })
//     }
// }
#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct IdxBoxHeapBak<Heap: ?Sized, Elem> {
    phantom: std::marker::PhantomData<(Elem, Heap)>,
    elems: slab::Slab<Elem>, // None is a tombstone
}
impl<Heap, T: Parse<Heap> + Copy> Parse<Heap> for CstfyTransparent<Heap, IdxBox<Heap, T>>
where
    Heap: SuperHeap<IdxBoxHeapBak<Heap, T>>,
{
    fn parse(
        source: &str,
        offset: parse::miette::SourceOffset,
        heap: &mut Self::Heap,
        errors: &mut Vec<parse::ParseError>,
    ) -> (
        Either<Heap, IdxBox<Heap, T>, std_parse_error::ParseError<Heap>>,
        parse::miette::SourceOffset,
    ) {
        let (it, new_offset) = <T as Parse<Heap>>::parse(source, offset, heap, errors);
        (
            cstfy_transparent_ok(IdxBox::construct(heap, it)),
            new_offset,
        )
    }

    fn unparse(&self, heap: &Self::Heap) -> Unparse {
        let item = uncstfy_transparent(self).unwrap();
        heap.subheap::<IdxBoxHeapBak<Heap, T>>().elems[item.idx as usize].unparse(heap)
    }
}
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = "L: Clone, R: Clone"))]
#[derivative(Copy(bound = "L: Copy, R: Copy"))]
pub enum Either<Heap, L, R> {
    Left(L, std::marker::PhantomData<Heap>),
    Right(R, std::marker::PhantomData<Heap>),
}
impl<Heap, L, R> Heaped for Either<Heap, L, R> {
    type Heap = Heap;
}
empty_heap_bak!(EitherHeapBak, L, R);
impl<
        Heap,
        L: TermRoundTrip<0, Core> + Heaped<Heap = Heap>,
        R: TermRoundTrip<0, Core> + Heaped<Heap = Heap>,
    > TermRoundTrip<0, Core> for Either<Heap, L, R>
{
    fn to_term(self, heap: &mut Heap) -> Term<0, Core, Heap> {
        match self {
            Either::Left(l, _) => {
                let l_term = l.to_term(heap);
                let fingerprint = TyFingerprint::from(stringify!(tymetafuncspec_core::Either));
                Term::new(fingerprint, vec![MaybeOpaqueTerm::Term(l_term)])
            }
            Either::Right(r, _) => {
                let r_term = r.to_term(heap);
                let fingerprint = TyFingerprint::from(stringify!(tymetafuncspec_core::Either));
                Term::new(fingerprint, vec![MaybeOpaqueTerm::Term(r_term)])
            }
        }
    }

    fn from_term(heap: &mut Heap, t: Term<0, Core, Heap>) -> Result<Self, FromTermError> {
        match &t.args[0] {
            MaybeOpaqueTerm::Term(t) => Ok(L::from_term(heap, t.clone())
                .map(|ok| Either::Left(ok, std::marker::PhantomData))
                .or_else(|_| {
                    R::from_term(heap, t.clone())
                        .map(|ok| Either::Right(ok, std::marker::PhantomData))
                })?),
            _ => Err(FromTermError::Invalid),
        }
    }
}
impl<Heap, L, R> parse::Parse<Heap>
    for CstfyTransparent<Heap, Either<Heap, CstfyTransparent<Heap, L>, CstfyTransparent<Heap, R>>>
where
    CstfyTransparent<Heap, L>: parse::Parse<Heap>,
    CstfyTransparent<Heap, R>: parse::Parse<Heap>,
{
    fn parse(
        source: &str,
        offset: parse::miette::SourceOffset,
        heap: &mut Self::Heap,
        errors: &mut Vec<parse::ParseError>,
    ) -> (
        CstfyTransparent<Heap, Either<Heap, CstfyTransparent<Heap, L>, CstfyTransparent<Heap, R>>>,
        parse::miette::SourceOffset,
    ) {
        let mut maybe_errors = vec![];
        let mut ute =
            match CstfyTransparent::<Heap, L>::parse(source, offset, heap, &mut maybe_errors) {
                (Either::Left(ok, _), new_offset) => {
                    return (
                        cstfy_transparent_ok(Either::Left(
                            Either::Left(ok, std::marker::PhantomData),
                            std::marker::PhantomData,
                        )),
                        new_offset,
                    )
                }
                (Either::Right(err, _), _) => Some(err),
            };
        match CstfyTransparent::<Heap, R>::parse(source, offset, heap, &mut maybe_errors) {
            (Either::Left(ok, _), new_offset) => {
                return (
                    cstfy_transparent_ok(Either::Right(
                        Either::Left(ok, std::marker::PhantomData),
                        std::marker::PhantomData,
                    )),
                    new_offset,
                )
            }
            (Either::Right(err, _), _) => {
                if ute.is_none() {
                    ute = Some(err);
                }
            }
        };
        errors.append(&mut maybe_errors);
        (
            Either::Right(ute.unwrap(), std::marker::PhantomData),
            offset,
        )
    }

    fn unparse(&self, heap: &Self::Heap) -> Unparse {
        match uncstfy_transparent(self) {
            Some(Either::Left(l, _)) => l.unparse(heap),
            Some(Either::Right(r, _)) => r.unparse(heap),
            _ => Unparse::new("err"),
        }
    }
}

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = "T: Clone"))]
#[derivative(Copy(bound = "T: Copy"))]
pub enum Maybe<Heap, T> {
    Just(T, std::marker::PhantomData<Heap>),
    Nothing,
}
impl<Heap, T> Heaped for Maybe<Heap, T> {
    type Heap = Heap;
}
empty_heap_bak!(MaybeHeapBak, T);
impl<Heap, T: TermRoundTrip<0, Core> + Heaped<Heap = Heap>> TermRoundTrip<0, Core>
    for Maybe<Heap, T>
{
    fn to_term(self, heap: &mut Heap) -> Term<0, Core, Heap> {
        match self {
            Maybe::Just(t, _) => {
                let t_term = t.to_term(heap);
                let fingerprint = TyFingerprint::from(stringify!(tymetafuncspec_core::Maybe));
                Term::new(fingerprint, vec![MaybeOpaqueTerm::Term(t_term)])
            }
            Maybe::Nothing => {
                let fingerprint = TyFingerprint::from(stringify!(tymetafuncspec_core::Maybe));
                Term::new(fingerprint, vec![])
            }
        }
    }

    fn from_term(heap: &mut Heap, t: Term<0, Core, Heap>) -> Result<Self, FromTermError> {
        match t.args.len() {
            0 => Ok(Maybe::Nothing),
            1 => Ok(Maybe::Just(
                T::from_term(heap, t.args[0].clone().to_term())?,
                std::marker::PhantomData,
            )),
            _ => Err(FromTermError::Invalid),
        }
    }
}
impl<Heap, T> parse::Parse<Heap> for Cstfy<Heap, Maybe<Heap, Cstfy<Heap, T>>>
where
    Cstfy<Heap, T>: parse::Parse<Heap>,
{
    fn parse(
        source: &str,
        offset: parse::miette::SourceOffset,
        heap: &mut Self::Heap,
        errors: &mut Vec<parse::ParseError>,
    ) -> (
        Cstfy<Heap, Maybe<Heap, Cstfy<Heap, T>>>,
        parse::miette::SourceOffset,
    ) {
        if source[offset.offset()..].starts_with("nothing") {
            return (
                cstfy_ok(
                    Maybe::Nothing,
                    offset,
                    parse::miette::SourceOffset::from(offset.offset() + 7),
                ),
                parse::miette::SourceOffset::from(offset.offset() + 7),
            );
        }
        let (it, new_offset) =
            <Cstfy<Heap, T> as parse::Parse<Heap>>::parse(source, offset, heap, errors);
        (
            cstfy_ok(
                Maybe::Just(it, std::marker::PhantomData),
                offset,
                new_offset,
            ),
            new_offset,
        )
    }

    fn unparse(&self, heap: &Self::Heap) -> Unparse {
        match uncstfy(self) {
            Some(Maybe::Just(t, _)) => t.unparse(heap),
            Some(Maybe::Nothing) => Unparse::new("nothing"),
            _ => Unparse::new("err"),
        }
    }
}
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = "L: Clone, R: Clone"))]
#[derivative(Copy(bound = "L: Copy, R: Copy"))]
pub struct Pair<Heap, L, R> {
    l: L,
    r: R,
    heap: std::marker::PhantomData<Heap>,
}
impl<Heap, L, R> Heaped for Pair<Heap, L, R> {
    type Heap = Heap;
}
empty_heap_bak!(PairHeapBak, L, R);
