use aspect::VisitationAspect;
use aspect::{Adtishness, NotAdtLike};
use ccf::CanonicallyConstructibleFrom;
use covisit::Covisit;
use parse_adt::{
    Lookahead, LookaheadAspect, ParseCursor, Parser,
    cstfy::{Cstfy, cstfy_ok},
};
use term::SuperHeap;
use words::InverseImplements;

use crate::{File, FileHeapBak};

impl<'a, Heap, L, Item, ItemLWord, FileMapped>
    Covisit<File<(), ItemLWord>, L, Cstfy<Heap, FileMapped>, Heap, NotAdtLike> for Parser<'a, L>
where
    Heap: SuperHeap<FileHeapBak<Heap, Item>>,
    Heap:
        InverseImplements<L, File<(), ItemLWord>, VisitationAspect, Implementor = File<Heap, Item>>,
    ItemLWord: Adtishness<VisitationAspect>,
    Parser<'a, L>:
        Covisit<ItemLWord, L, Item, Heap, <ItemLWord as Adtishness<VisitationAspect>>::X>,
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
impl<ItemLWord> Lookahead<NotAdtLike> for File<(), ItemLWord> {
    fn matches(_: &ParseCursor<'_>) -> bool {
        true
    }
}

pub fn file<Heap, L, Item, ItemLWord>(source: &str) -> (Heap, Cstfy<Heap, File<Heap, Item>>)
where
    Heap: Default,
    for<'a> Parser<'a, L>:
        covisit::Covisit<File<(), ItemLWord>, L, Cstfy<Heap, File<Heap, Item>>, Heap, NotAdtLike>,
{
    let mut parser = parse_adt::Parser::new(source);
    let mut heap = Heap::default();
    let ret = <Parser<'_, L> as covisit::Covisit<_, _, _, _, _>>::covisit(&mut parser, &mut heap);
    (heap, ret)
}
