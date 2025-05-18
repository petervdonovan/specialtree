use ccf::CanonicallyConstructibleFrom;
use covisit::Covisit;
use parse_adt::{
    Lookahead, ParseCursor, Parser,
    cstfy::{Cstfy, cstfy_ok},
};
use pmsp::{AdtMetadata, TmfMetadata};
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
impl<Heap, L, Item> Lookahead<Heap, L> for File<Heap, Item> {
    fn matches(_: &ParseCursor<'_>) -> bool {
        true
    }
}

pub fn file<Heap: Default, L, Item>(source: &str) -> (Heap, Cstfy<Heap, File<Heap, Item>>)
where
    for<'a> Parser<'a, L>: covisit::Covisit<
            TmfMetadata<File<Heap, Item>, (AdtMetadata, ())>,
            Cstfy<Heap, File<Heap, Item>>,
            Heap,
            L,
        >,
{
    let mut parser = parse_adt::Parser::new(source);
    let mut heap = Heap::default();
    let ret = <Parser<'_, L> as covisit::Covisit<_, _, _, _>>::covisit(&mut parser, &mut heap);
    (heap, ret)
}
