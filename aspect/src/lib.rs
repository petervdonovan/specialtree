use std::any::Any;

use syn::parse_quote;

pub struct AdtLike;
pub struct NotAdtLike;
pub trait AdtLikeOrNot {}
impl AdtLikeOrNot for AdtLike {}
impl AdtLikeOrNot for NotAdtLike {}

pub trait Aspect: Any {
    fn zst_path(&self) -> syn::Path;
    fn static_ref(&self) -> &'static mut Self
    where
        Self: Sized,
    {
        assert!(std::mem::size_of_val(self) == 0);
        unsafe { &mut *std::ptr::NonNull::dangling().as_mut() }
    }
}

pub trait Adtishness<A: Aspect> {
    type X: AdtLikeOrNot;
}

pub struct VisitationAspect;
impl Aspect for VisitationAspect {
    fn zst_path(&self) -> syn::Path {
        parse_quote! {
            aspect::VisitationAspect
        }
    }
}
