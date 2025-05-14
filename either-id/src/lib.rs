use functor_derive::Functor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Functor, Serialize, Deserialize, Hash, PartialOrd, Ord)]
#[functor(L as l, R as r)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
