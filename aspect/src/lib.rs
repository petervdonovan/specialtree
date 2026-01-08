use std::any::Any;

use syn::parse_quote;

pub struct AdtLike;
pub struct NotAdtLike;
pub trait AdtLikeOrNot {}
impl AdtLikeOrNot for AdtLike {}
impl AdtLikeOrNot for NotAdtLike {}

pub trait Aspect: Any {
    fn zst_path(&self) -> syn::Path;
}

pub trait Adtishness<A: Aspect> {
    type X: AdtLikeOrNot;
}

pub struct Visitation;
impl Aspect for Visitation {
    fn zst_path(&self) -> syn::Path {
        parse_quote! {
            aspect::Visitation
        }
    }
}
