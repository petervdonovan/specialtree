use ccf::CanonicallyConstructibleFrom;
use pmsp::{AdtMetadata, TmfMetadata};
// use visit::Visit;
use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Set, SetHeapBak};

use crate::{Unparser, Visit};

impl<'arena, Heap, L, MappedBNat: Copy> Visit<TmfMetadata<BoundedNat<Heap>>, MappedBNat, Heap, L>
    for Unparser<'arena, L>
where
    MappedBNat: CanonicallyConstructibleFrom<Heap, (BoundedNat<Heap>, ())>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedBNat) {
        let (nat, ()) = (*t).deconstruct(heap);
        self.unparse.dynamic_text(nat.n.to_string());
    }
}

impl<'arena, Heap, L, Elem, MappedSet: Copy> Visit<TmfMetadata<Set<Heap, Elem>>, MappedSet, Heap, L>
    for Unparser<'arena, L>
where
    Heap: term::SuperHeap<SetHeapBak<Heap, Elem>>,
    MappedSet: CanonicallyConstructibleFrom<Heap, (Set<Heap, Elem>, ())>,
    Unparser<'arena, L>: Visit<AdtMetadata, Elem, Heap, L>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedSet) {
        let (set, ()) = (*t).deconstruct(heap);
        self.unparse.static_text("{");
        for item in set.iter(heap) {
            //     self.visit(heap, item);
        }
        self.unparse.static_text("}");
    }
}

impl<'arena, Heap, L, Elem, MappedIdxBox: Copy>
    Visit<TmfMetadata<IdxBox<Heap, Elem>>, MappedIdxBox, Heap, L> for Unparser<'arena, L>
where
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
    MappedIdxBox: CanonicallyConstructibleFrom<Heap, (IdxBox<Heap, Elem>, ())>,
    Unparser<'arena, L>: Visit<AdtMetadata, Elem, Heap, L>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedIdxBox) {
        let (ib, ()) = (*t).deconstruct(heap);
        let elem = ib.get(heap);
        self.visit(heap, elem);
    }
}
