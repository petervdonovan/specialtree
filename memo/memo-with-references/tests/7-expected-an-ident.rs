use memo_with_references::memo;

#[memo(123)]
fn memome(cache: &'a str) {}

fn main() {}
