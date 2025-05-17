use ccf::CanonicallyConstructibleFrom;
use pmsp::TmfMetadata;
use term::SuperHeap;
use unparse_adt::Unparser;
use visit::Visit;

use crate::{File, FileHeapBak};

impl<'a, Heap, L, Item, ItemTmfMetadata, FileMapped: Copy>
    Visit<TmfMetadata<File<Heap, Item>, (ItemTmfMetadata, ())>, FileMapped, Heap, L>
    for Unparser<'a, L>
where
    Heap: SuperHeap<FileHeapBak<Heap, Item>>,
    Unparser<'a, L>: Visit<ItemTmfMetadata, Item, Heap, L>,
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
