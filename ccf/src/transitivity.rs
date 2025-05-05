use crate::{CanonicallyConstructibleFrom, DirectlyCanonicallyConstructibleFrom};

pub trait TransitivelyUnitCcf<Heap, T> {
    type Intermediary;
}

pub trait TransitivelyAllCcf<Heap, T> {
    type Intermediary;
    type Intermediaries;
}

impl<Heap, T, U> CanonicallyConstructibleFrom<Heap, (T, ())> for U
where
    U: Copy,
    U: TransitivelyUnitCcf<Heap, T>,
    U: DirectlyCanonicallyConstructibleFrom<Heap, (U::Intermediary, ())>,
    U::Intermediary: CanonicallyConstructibleFrom<Heap, (T, ())>,
{
    fn construct(heap: &mut Heap, t: (T, ())) -> Self {
        let intermediary = U::Intermediary::construct(heap, t);
        Self::construct(heap, (intermediary, ()))
    }

    fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
        self.deconstruct_succeeds(heap) && self.deconstruct(heap).0.deconstruct_succeeds(heap)
    }

    fn deconstruct(self, heap: &Heap) -> (T, ()) {
        self.deconstruct(heap).0.deconstruct(heap)
    }
}

pub trait AllCcf<Heap, FromConsList> {
    fn construct(heap: &mut Heap, t: FromConsList) -> Self;
    fn deconstruct_succeeds(&self, heap: &Heap) -> bool;
    fn deconstruct(self, heap: &Heap) -> FromConsList;
}
impl<Heap> AllCcf<Heap, ()> for () {
    fn construct(_heap: &mut Heap, _t: ()) -> Self {}

    fn deconstruct_succeeds(&self, _heap: &Heap) -> bool {
        true
    }

    fn deconstruct(self, _heap: &Heap) {}
}
impl<Heap, TCar, TCdr, UCar, UCdr> AllCcf<Heap, (TCar, TCdr)> for (UCar, UCdr)
where
    UCar: CanonicallyConstructibleFrom<Heap, (TCar, ())>,
    UCdr: AllCcf<Heap, TCdr>,
{
    fn construct(heap: &mut Heap, t: (TCar, TCdr)) -> Self {
        let (car, cdr) = t;
        let ucar = UCar::construct(heap, (car, ()));
        let ucdr = UCdr::construct(heap, cdr);
        (ucar, ucdr)
    }

    fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
        self.0.deconstruct_succeeds(heap) && self.1.deconstruct_succeeds(heap)
    }

    fn deconstruct(self, heap: &Heap) -> (TCar, TCdr) {
        let (car, cdr) = self;
        let tcar = car.deconstruct(heap);
        let tcdr = cdr.deconstruct(heap);
        (tcar.0, tcdr)
    }
}

impl<Heap, TCar, TCdrCar, TCdrCdr, U> CanonicallyConstructibleFrom<Heap, (TCar, (TCdrCar, TCdrCdr))>
    for U
where
    U: Copy,
    U: TransitivelyAllCcf<Heap, (TCar, (TCdrCar, TCdrCdr))>,
    U::Intermediaries: AllCcf<Heap, (TCar, (TCdrCar, TCdrCdr))>,
    U: CanonicallyConstructibleFrom<Heap, (U::Intermediary, ())>,
    U::Intermediary: DirectlyCanonicallyConstructibleFrom<Heap, U::Intermediaries>,
{
    fn construct(heap: &mut Heap, t: (TCar, (TCdrCar, TCdrCdr))) -> Self {
        let (car, cdr) = t;
        let intermediaries = U::Intermediaries::construct(heap, (car, cdr));
        let intermediary = U::Intermediary::construct(heap, intermediaries);
        Self::construct(heap, (intermediary, ()))
    }

    fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
        self.deconstruct_succeeds(heap) && self.deconstruct(heap).0.deconstruct_succeeds(heap)
    }

    fn deconstruct(self, heap: &Heap) -> (TCar, (TCdrCar, TCdrCdr)) {
        let intermediary = self.deconstruct(heap);
        let intermediaries = U::Intermediary::deconstruct(intermediary.0, heap);
        let (car, cdr) = intermediaries.deconstruct(heap);
        (car, cdr)
    }
}
