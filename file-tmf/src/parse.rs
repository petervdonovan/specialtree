use ccf::CanonicallyConstructibleFrom;
use covisit::Covisit;
use parse_adt::{
    Parser,
    cstfy::{Cstfy, cstfy_ok},
};
use pmsp::TmfMetadata;
use term::SuperHeap;

use crate::{File, FileHeapBak};

impl<'a, Heap, L, Item, ItemTmfMetadata, FileMapped>
    Covisit<TmfMetadata<File<Heap, Item>, (ItemTmfMetadata, ())>, Cstfy<Heap, FileMapped>, Heap, L>
    for Parser<'a, L>
where
    Heap: SuperHeap<FileHeapBak<Heap, Item>>,
    Parser<'a, L>: Covisit<ItemTmfMetadata, Item, Heap, L>,
    FileMapped: CanonicallyConstructibleFrom<Heap, (File<Heap, Item>, ())>,
{
    fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, FileMapped> {
        let previous_offset = self.pc.position;
        let mut items = vec![];
        while let Some(_) = self.pc.peek_words().next() {
            items.push(self.covisit(heap));
        }
        let subheap = heap.subheap_mut::<FileHeapBak<_, _>>();
        let file = File {
            // name: None,
            items: subheap.vecs.insert(items),
            phantom: Default::default(),
        };
        cstfy_ok(
            FileMapped::construct(heap, (file, ())),
            previous_offset,
            self.pc.position,
        )
    }
}
