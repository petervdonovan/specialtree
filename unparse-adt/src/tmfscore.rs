use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Set, SetHeapBak};

use crate::Unparser;

impl<Heap, Pmsp, L, Amc, DepthFuelUpperBits, DepthFuelLastBit, Fnlut>
    term::visit::Visitable<
        Unparser<'_, L, Amc>,
        Pmsp,
        Heap,
        typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
        Fnlut,
    > for BoundedNat<Heap>
{
    fn visit(&self, visitor: &mut Unparser<'_, L, Amc>, _: &mut &Heap, _: Fnlut) {
        visitor.unparse.dynamic_text(self.n.to_string());
    }
}

impl<'a, Heap, Elem, Pmsp, L, DepthFuelUpperBits, DepthFuelLastBit, Fnlut: Copy>
    term::visit::Visitable<
        Unparser<'a, L, ()>,
        Pmsp,
        Heap,
        typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
        Fnlut,
    > for Set<Heap, Elem>
where
    typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
    Elem: term::visit::Visitable<
            Unparser<'a, L, ()>,
            Pmsp,
            Heap,
            typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
            Fnlut,
        >,
    Heap: term::SuperHeap<SetHeapBak<Heap, Elem>>,
{
    fn visit(&self, visitor: &mut Unparser<'_, L, ()>, _: &mut &Heap, _: Fnlut) {
        todo!()
    }
}

impl<'a, Heap, Elem, Pmsp, L, DepthFuelUpperBits, DepthFuelLastBit, Fnlut>
    term::visit::Visitable<
        Unparser<'a, L, ()>,
        Pmsp,
        Heap,
        typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
        Fnlut,
    > for IdxBox<Heap, Elem>
where
    typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
    Elem: term::visit::Visitable<
            Unparser<'a, L, ()>,
            Pmsp,
            Heap,
            typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
            Fnlut,
        >,
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
{
    fn visit(&self, visitor: &mut Unparser<'_, L, ()>, _: &mut &Heap, _: Fnlut) {
        // let initial_offset = visitor.position;
        // let item = Cstfy::<Heap, Elem>::co_visit(visitor, heap, fnlut);
        // let final_offset = visitor.position;
        // cstfy_ok(IdxBox::new(heap, item), initial_offset, final_offset)
        todo!()
    }
}
