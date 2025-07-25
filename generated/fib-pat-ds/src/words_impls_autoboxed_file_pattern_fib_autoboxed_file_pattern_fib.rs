#![feature(custom_inner_attributes)]
#![rustfmt::skip]
#![allow(warnings)]
#![allow(unknown_lints)]
//! @generated by [words::words_impls]
use tymetafuncspec_core::Set;
use tymetafuncspec_core::IdxBox;
use tymetafuncspec_core::BoundedNat;
use pattern_tmf::OrVariableZeroOrMore;
use pattern_tmf::OrVariable;
use pattern_tmf::NamedPattern;
use file_tmf::File;
use crate::words_mod_autoboxed_file_pattern_fib as wmafpf;
use crate::words_mod_autoboxed_file_pattern_fib::sorts as wmafpfs;
use crate::term_specialized_autoboxed_file_pattern_fib as tsafpf;
impl words::Implements<tsafpf::Heap, wmafpf::L> for tsafpf::Plus {
    type LWord = wmafpfs::Plus;
}
impl words::Implements<tsafpf::Heap, wmafpf::L> for tsafpf::LeftOperand {
    type LWord = wmafpfs::LeftOperand;
}
impl words::Implements<tsafpf::Heap, wmafpf::L> for tsafpf::RightOperand {
    type LWord = wmafpfs::RightOperand;
}
impl words::Implements<tsafpf::Heap, wmafpf::L> for tsafpf::F {
    type LWord = wmafpfs::F;
}
impl words::Implements<tsafpf::Heap, wmafpf::L> for tsafpf::Sum {
    type LWord = wmafpfs::Sum;
}
impl words::Implements<tsafpf::Heap, wmafpf::L> for tsafpf::Nat {
    type LWord = wmafpfs::Nat;
}
impl words::Implements<tsafpf::Heap, wmafpf::L> for tsafpf::FileItem {
    type LWord = wmafpfs::FileItem;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for OrVariable<tsafpf::Heap, tsafpf::LeftOperand> {
    type LWord = OrVariable<(), wmafpfs::LeftOperand>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for OrVariable<tsafpf::Heap, tsafpf::RightOperand> {
    type LWord = OrVariable<(), wmafpfs::RightOperand>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for OrVariable<tsafpf::Heap, tsafpf::Nat> {
    type LWord = OrVariable<(), wmafpfs::Nat>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for OrVariable<
    tsafpf::Heap,
    Set<tsafpf::Heap, OrVariableZeroOrMore<tsafpf::Heap, tsafpf::Nat>>,
> {
    type LWord = OrVariable<(), Set<(), OrVariableZeroOrMore<(), wmafpfs::Nat>>>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for Set<tsafpf::Heap, OrVariableZeroOrMore<tsafpf::Heap, tsafpf::Nat>> {
    type LWord = Set<(), OrVariableZeroOrMore<(), wmafpfs::Nat>>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for OrVariableZeroOrMore<tsafpf::Heap, tsafpf::Nat> {
    type LWord = OrVariableZeroOrMore<(), wmafpfs::Nat>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L> for BoundedNat<tsafpf::Heap> {
    type LWord = BoundedNat<()>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L> for IdxBox<tsafpf::Heap, tsafpf::F> {
    type LWord = IdxBox<(), wmafpfs::F>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L> for IdxBox<tsafpf::Heap, tsafpf::Plus> {
    type LWord = IdxBox<(), wmafpfs::Plus>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for NamedPattern<tsafpf::Heap, tsafpf::Plus> {
    type LWord = NamedPattern<(), wmafpfs::Plus>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for NamedPattern<tsafpf::Heap, tsafpf::LeftOperand> {
    type LWord = NamedPattern<(), wmafpfs::LeftOperand>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for NamedPattern<tsafpf::Heap, tsafpf::RightOperand> {
    type LWord = NamedPattern<(), wmafpfs::RightOperand>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for NamedPattern<tsafpf::Heap, tsafpf::F> {
    type LWord = NamedPattern<(), wmafpfs::F>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for NamedPattern<tsafpf::Heap, tsafpf::Sum> {
    type LWord = NamedPattern<(), wmafpfs::Sum>;
}
impl words::Implements<tsafpf::Heap, wmafpf::L>
for File<tsafpf::Heap, tsafpf::FileItem> {
    type LWord = File<(), wmafpfs::FileItem>;
}
