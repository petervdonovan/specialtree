use memo_with_references::memo;

#[memo(cache)]
fn memome<'a>(cache: &'a str, data: &mut String) {}

fn main() {}
