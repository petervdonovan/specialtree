#![allow(incomplete_features)]

pub struct AdtLike;
pub struct NotAdtLike;
pub trait AdtLikeOrNot {}
impl AdtLikeOrNot for AdtLike {}
impl AdtLikeOrNot for NotAdtLike {}

pub trait Aspect {}

pub trait Adtishness<A: Aspect> {
    type X: AdtLikeOrNot;
}

pub struct Visitation;
impl Aspect for Visitation {}