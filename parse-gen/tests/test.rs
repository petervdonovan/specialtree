use cst::data_structure::Heap;

include!("fib_parse.rs");

fn round_trip<T>(input: &str)
where
    tymetafuncspec_core::Cstfy<Heap, T>: ::parse::Parse<Heap>,
{
    let mut heap = Heap::default();
    let mut errors = Vec::new();
    let (nl, offset) = <tymetafuncspec_core::Cstfy<Heap, T> as ::parse::Parse<Heap>>::parse(
        input,
        0.into(),
        &mut heap,
        &mut errors,
    );
    println!(
        "parse result: {}",
        <tymetafuncspec_core::Cstfy<Heap, T> as ::parse::Parse<Heap>>::unparse(&nl, &heap)
    );
}

fn round_trip_transparent<T>(input: &str)
where
    tymetafuncspec_core::CstfyTransparent<Heap, T>: ::parse::Parse<Heap>,
{
    let mut heap = Heap::default();
    let mut errors = Vec::new();
    let (nl, offset) =
        <tymetafuncspec_core::CstfyTransparent<Heap, T> as ::parse::Parse<Heap>>::parse(
            input,
            0.into(),
            &mut heap,
            &mut errors,
        );
    println!(
        "parse result: {}",
        <tymetafuncspec_core::CstfyTransparent<Heap, T> as ::parse::Parse<Heap>>::unparse(
            &nl, &heap
        )
    );
}

#[test]
fn main() {
    round_trip::<tymetafuncspec_core::BoundedNat<Heap>>("23");
    round_trip_transparent::<cst::data_structure::Nat>("23");
}
