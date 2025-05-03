use term::case_split::HasBorrowedHeapRef;
use unparse::Unparse;

pub mod tmfscore;
pub mod unparse;

pub struct Unparser<'arena, CurrentNode> {
    pub unparse: Unparse<'arena>,
    phantom: std::marker::PhantomData<CurrentNode>,
}

impl<'arena, C> HasBorrowedHeapRef for Unparser<'arena, C> {
    type Borrowed<'a, Heap: 'a> = &'a Heap;
    fn with_decayed_heapref<Heap, T, F: Fn(&Heap) -> T>(
        heap: &mut Self::Borrowed<'_, Heap>,
        f: F,
    ) -> T {
        f(heap)
    }
}
