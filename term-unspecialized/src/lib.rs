use langspec::{langspec::SortId, tymetafunc::TyMetaFuncSpec};
use term::{CanonicallyConstructibleFrom, Heaped, TyFingerprint};

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
/// The fingerprint must be known at runtime OR compile time, but not
/// necessarily both. The compile-time value of 0 is a sentinel value indicating
/// that the fingerprint is not known at compile time; when it is zero, the
/// fingerprint must be known at run time. Collisions with the sentinel value
/// will not occur in practice as long as a reasonable-quality hash
/// implementation is used. Before construction, the compile-time value is
/// needed because the run-time value has not been computed yet.
pub struct Term<const TOP_FINGERPRINT: u128, Tmfs: TyMetaFuncSpec, Heap> {
    pub tyfingerprint: TyFingerprint,
    pub args: Vec<Term<0, Tmfs, Heap>>,
    phantom: std::marker::PhantomData<(Tmfs, Heap)>,
}
impl<const TOP_FINGERPRINT: u128, Tmfs: TyMetaFuncSpec, Heap> Term<TOP_FINGERPRINT, Tmfs, Heap> {
    pub fn new(sortid: TyFingerprint, args: Vec<Term<0, Tmfs, Heap>>) -> Self {
        (Term::<0, Tmfs, Heap> {
            tyfingerprint: sortid,
            args,
            phantom: std::marker::PhantomData,
        })
        .dynamic_checked_cast()
    }
}
pub trait TermRoundTrip<const TOP_FINGERPRINT: u128, Tmfs: TyMetaFuncSpec>: Heaped {
    fn to_term(self, heap: &mut Self::Heap) -> Term<TOP_FINGERPRINT, Tmfs, Self::Heap>;
    fn from_term(heap: &mut Self::Heap, t: Term<TOP_FINGERPRINT, Tmfs, Self::Heap>) -> Self;
}
impl<const TOP_FINGERPRINT: u128, Tmfs: TyMetaFuncSpec, Heap> Term<TOP_FINGERPRINT, Tmfs, Heap> {
    pub fn dynamic_checked_cast<const TOP_FINGERPRINT2: u128>(
        self,
    ) -> Term<TOP_FINGERPRINT2, Tmfs, Heap> {
        if self.tyfingerprint != TyFingerprint::from(TOP_FINGERPRINT2) {
            panic!("Dynamic cast failed");
        }
        Term::<TOP_FINGERPRINT2, Tmfs, Heap> {
            tyfingerprint: self.tyfingerprint,
            args: self.args,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn compile_time_forget(self) -> Term<0, Tmfs, Heap> {
        Term::<0, Tmfs, Heap> {
            tyfingerprint: self.tyfingerprint,
            args: self.args,
            phantom: std::marker::PhantomData,
        }
    }
}
impl<const FROM_FINGERPRINT: u128, const TO_FINGERPRINT: u128, Tmfs: TyMetaFuncSpec, Heap>
    TermRoundTrip<TO_FINGERPRINT, Tmfs> for Term<FROM_FINGERPRINT, Tmfs, Heap>
{
    fn to_term(self, _: &mut Self::Heap) -> Term<TO_FINGERPRINT, Tmfs, Heap> {
        if !(FROM_FINGERPRINT == 0 || TO_FINGERPRINT == 0) && FROM_FINGERPRINT != TO_FINGERPRINT {
            panic!("Both compile-time fingerprints are known and are not equal, so one of them must be wrong");
        }
        if self.tyfingerprint != TyFingerprint::from(TO_FINGERPRINT) {
            panic!("Dynamic cast failed");
        }
        Term::<TO_FINGERPRINT, Tmfs, Heap> {
            tyfingerprint: self.tyfingerprint,
            args: self.args,
            phantom: std::marker::PhantomData,
        }
    }

    fn from_term(_: &mut Self::Heap, t: Term<TO_FINGERPRINT, Tmfs, Heap>) -> Self {
        if !(FROM_FINGERPRINT == 0 || TO_FINGERPRINT == 0) && FROM_FINGERPRINT != TO_FINGERPRINT {
            panic!("Both compile-time fingerprints are known and are not equal, so one of them must be wrong");
        }
        if t.tyfingerprint != TyFingerprint::from(FROM_FINGERPRINT) {
            panic!("Dynamic cast failed");
        }
        Term::<FROM_FINGERPRINT, Tmfs, Heap> {
            tyfingerprint: t.tyfingerprint,
            args: t.args,
            phantom: std::marker::PhantomData,
        }
    }
}

// #[derive(Clone)]
// pub struct TermHeap {}
// impl<const TOP_FINGERPRINT: u128, Tmfs: TyMetaFuncSpec> Heaped for Term<TOP_FINGERPRINT, Tmfs> {
//     type Heap = TermHeap;
// }
impl<const TOP_FINGERPRINT: u128, Tmfs: TyMetaFuncSpec, Heap> Heaped
    for Term<TOP_FINGERPRINT, Tmfs, Heap>
{
    type Heap = Heap;
}
macro_rules! replace_expr {
    ($_t:tt $sub:tt) => {
        $sub
    };
    ($_t:tt $sub:ty) => {
        $sub
    };
}
macro_rules! ccf {
    ($($indices:expr),*) => {
        paste::paste! {
            impl<const TOP_FINGERPRINT: u128, Heap, $([< To0Term $indices >]: TermRoundTrip<0, Tmfs> + Heaped<Heap=Heap>,)* Tmfs: TyMetaFuncSpec>
                CanonicallyConstructibleFrom<($(replace_expr!($indices [< To0Term $indices >]),)*)> for Term<TOP_FINGERPRINT, Tmfs, Heap>
            {
                fn construct(heap: &mut Heap, t: ($(replace_expr!($indices [< To0Term $indices >]),)*)) -> Self {
                    Term {
                        tyfingerprint: TyFingerprint::from(TOP_FINGERPRINT),
                        args: vec![$(t.$indices.to_term(heap)),*],
                        phantom: std::marker::PhantomData,
                    }
                }

                fn deconstruct(self, heap: &mut Self::Heap) -> ($(replace_expr!($indices [< To0Term $indices >]),)*) {
                    let mut args = self.args;
                    args.reverse();
                    ($({
                        replace_expr!($indices ([< To0Term $indices >]::from_term(heap, args.pop().unwrap())))
                    },)*)
                }
            }
        }
    };
}
ccf!(0);
ccf!(0, 1);
ccf!(0, 1, 2);
ccf!(0, 1, 2, 3);
ccf!(0, 1, 2, 3, 4);
ccf!(0, 1, 2, 3, 4, 5);
ccf!(0, 1, 2, 3, 4, 5, 6);
ccf!(0, 1, 2, 3, 4, 5, 6, 7);
ccf!(0, 1, 2, 3, 4, 5, 6, 7, 8);
ccf!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
ccf!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
ccf!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
ccf!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
ccf!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
ccf!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
ccf!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
ccf!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
