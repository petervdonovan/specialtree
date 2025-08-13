use memo_with_references::memo;

#[memo(cache other)]
fn memome(cache: &'a str) {}

fn main() {}
