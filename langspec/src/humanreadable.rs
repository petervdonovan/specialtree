use serde::{Deserialize, Serialize};

use crate::langspec::Name;

#[derive(Debug, Serialize, Deserialize)]
pub struct LangSpecHuman {
    pub name: Name,
    pub products: Vec<Product>,
    pub sums: Vec<Sum>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: Name,
    pub sorts: Vec<SortId>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sum {
    pub name: Name,
    pub sorts: Vec<SortId>,
}
pub type SortId = crate::langspec::SortId<String>;
