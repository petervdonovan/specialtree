use functor_derive::Functor;
use langspec::langspec::ToLiteral;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Functor, Serialize, Deserialize)]
#[functor(L as l, R as r)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: ToLiteral, R: ToLiteral> ToLiteral for Either<L, R> {
    fn to_literal(&self) -> syn::Expr {
        match self {
            Self::Left(l) => {
                let l = l.to_literal();
                syn::parse_quote! {
                    extension_everywhere_alternative::Either::Left(#l)
                }
            }
            Self::Right(r) => {
                let r = r.to_literal();
                syn::parse_quote! {
                    extension_everywhere_alternative::Either::Right(#r)
                }
            }
        }
    }
}
