use memo_with_references::memo;

#[memo(cache)]
fn memome<'a>(cache: &'a str) {}

fn main() {}
