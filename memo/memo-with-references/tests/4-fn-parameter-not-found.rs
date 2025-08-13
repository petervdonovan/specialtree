use memo_with_references::memo;

#[memo(nonexistent)]
fn memome(cache: &'a str) {}

fn main() {}
