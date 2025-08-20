use ccf::CanonicallyConstructibleFrom;
use derivative::Derivative;
use langspec::tymetafunc::{ArgId, RustTyMap, TyMetaFuncData, TyMetaFuncSpec};
use pmsp::Visitation;
use serde::{Deserialize, Serialize};
use term::{SuperHeap, TyMetaFunc};
use tree_identifier::Identifier;
use words::{Adtishness, NotAdtLike};

pub mod parse;
pub mod unparse;

pub struct FileTmfs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct FileTmfId;

impl TyMetaFuncSpec for FileTmfs {
    type TyMetaFuncId = FileTmfId;

    fn ty_meta_func_data(_: &Self::TyMetaFuncId) -> langspec::tymetafunc::TyMetaFuncData {
        TyMetaFuncData {
            name: Identifier::from_camel_str("File").unwrap(),
            args: Box::new([Identifier::from_camel_str("Item").unwrap()]),
            imp: RustTyMap {
                ty_func: syn::parse_quote! { file_tmf::File },
            },
            heapbak: RustTyMap {
                ty_func: syn::parse_quote! { file_tmf::FileHeapBak },
            },
            idby: langspec::tymetafunc::IdentifiedBy::Tmf,
            canonical_froms: Box::new([]),
            size_depends_on: Box::new([]),
            is_collection_of: Box::new([ArgId(0)]),
            transparency: langspec::tymetafunc::Transparency::Visible,
        }
    }
}
// type Filename = <string_interner::DefaultBackend as string_interner::backend::Backend>::Symbol;
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Copy(bound = ""))]
pub struct File<Heap, Item> {
    // name: Option<Filename>,
    items: usize,
    phantom: std::marker::PhantomData<(Heap, Item)>,
}
impl<Heap, Item> TyMetaFunc for File<Heap, Item> {
    type HeapBak = FileHeapBak<Heap, Item>;
}
#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct FileHeapBak<Heap, Item> {
    // names: string_interner::DefaultStringInterner,
    vecs: slab::Slab<std::vec::Vec<Item>>,
    phantom: std::marker::PhantomData<Heap>,
}
pub fn items<'a, Heap, FileMapped, Item>(
    heap: &'a Heap,
    f: &FileMapped,
) -> impl Iterator<Item = &'a Item>
where
    FileMapped: Copy + CanonicallyConstructibleFrom<Heap, (File<Heap, Item>, ())>,
    Heap: SuperHeap<FileHeapBak<Heap, Item>>,
{
    let (f, ()) = f.deconstruct(heap);
    let subheap = heap.subheap::<FileHeapBak<Heap, Item>>();
    let items = subheap.vecs.get(f.items).unwrap();
    items.iter()
}
impl<ItemLWord> Adtishness<Visitation> for File<(), ItemLWord> {
    type X = NotAdtLike;
}
