#![allow(clippy::needless_question_mark)]
use functor_derive::Functor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Functor)]
#[functor(T as t)]
pub struct BoundedSlice<T, const S: usize>(u16, [Option<T>; S]);

impl<T: Copy, const S: usize> Serialize for BoundedSlice<T, S>
where
    T: Serialize,
{
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: serde::Serializer,
    {
        let v: Vec<T> = (*self).into();
        v.serialize(serializer)
    }
}

impl<'de, T: Copy, const S: usize> Deserialize<'de> for BoundedSlice<T, S>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <Vec<T> as Deserialize<'de>>::deserialize(deserializer).map(|v| v.try_into().unwrap())
    }
}

impl<T: Copy, const S: usize> BoundedSlice<T, S> {
    pub fn new(slice: &[T]) -> Self {
        let len = slice.len() as u16;
        let mut array = [None; S];
        for (i, item) in slice.iter().enumerate() {
            array[i] = Some(*item);
        }
        BoundedSlice(len, array)
    }
}

impl<T: Copy, const S: usize> TryFrom<BoundedSlice<T, S>> for [T; S] {
    type Error = &'static str;

    fn try_from(value: BoundedSlice<T, S>) -> Result<Self, Self::Error> {
        if value.0 as usize != value.len() {
            Err("Length mismatch")
        } else {
            let mut array = [None; S];
            for (i, item) in value.1.into_iter().enumerate() {
                array[i] = item;
            }
            Ok(array.map(|x| x.unwrap()))
        }
    }
}

impl<T: Copy, const S: usize> TryFrom<Vec<T>> for BoundedSlice<T, S> {
    type Error = &'static str;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.len() > S {
            Err("Vector is too large")
        } else {
            let len = vec.len() as u16;
            let mut array = [None; S];
            for (i, item) in vec.into_iter().enumerate() {
                array[i] = Some(item);
            }
            Ok(BoundedSlice(len, array))
        }
    }
}

impl<T: Copy, const S: usize> From<BoundedSlice<T, S>> for Vec<T> {
    fn from(value: BoundedSlice<T, S>) -> Self {
        value.1.iter().flatten().copied().collect()
    }
}

impl<T, const S: usize> BoundedSlice<T, S> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.1.iter().flatten()
    }
    pub fn len(&self) -> usize {
        self.0 as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// impl<T, const S: usize> BoundedSlice<T, S> {
//     pub fn from_vc() -> Self
// }
