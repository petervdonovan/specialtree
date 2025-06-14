use ccf::{CanonicallyConstructibleFrom, VisitationInfo};
use term::SuperHeap;
use unparse_adt::Unparser;
use visit::Visit;
use words::{InverseImplements, NotAdtLike};

use crate::{File, FileHeapBak};

impl<'a, Heap, L, Item, ItemLWord, FileMapped: Copy>
    Visit<File<(), ItemLWord>, L, FileMapped, Heap, NotAdtLike> for Unparser<'a, L>
where
    Heap: SuperHeap<FileHeapBak<Heap, Item>>,
    Heap: InverseImplements<L, File<(), ItemLWord>, ExternBehavioralImplementor = File<Heap, Item>>,
    ItemLWord: VisitationInfo,
    Unparser<'a, L>: Visit<ItemLWord, L, Item, Heap, <ItemLWord as VisitationInfo>::AdtLikeOrNot>,
    FileMapped: CanonicallyConstructibleFrom<Heap, (File<Heap, Item>, ())>,
{
    fn visit(&mut self, heap: &Heap, t: &FileMapped) {
        let subheap = heap.subheap::<FileHeapBak<_, _>>();
        let (f, ()) = t.deconstruct(heap);
        let items = subheap.vecs.get(f.items).unwrap();
        self.unparse.consistent_group_start();
        for item in items {
            self.visit(heap, item);
            self.unparse.linebreak();
        }
        self.unparse.group_end();
    }
}

pub fn file<L, Heap, Item, ItemLWord, FileMapped>(heap: &Heap, t: &FileMapped) -> String
where
    FileMapped: CanonicallyConstructibleFrom<Heap, (File<Heap, Item>, ())>,
    for<'a> Unparser<'a, L>: Visit<File<(), ItemLWord>, L, FileMapped, Heap, NotAdtLike>,
{
    let arena = bumpalo::Bump::new();
    let mut unparser = Unparser::new(&arena);
    <Unparser<'_, L> as Visit<_, _, _, _, _>>::visit(&mut unparser, heap, t);
    format!("{}", &unparser.unparse)
}
