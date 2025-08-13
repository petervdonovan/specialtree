use memo_with_references::memo;

#[memo(cache)]
fn test_multiple_same_lifetime<'a, 'b>(cache: &'a str, data: &'a str, other: &'b str) {}

fn main() {}
