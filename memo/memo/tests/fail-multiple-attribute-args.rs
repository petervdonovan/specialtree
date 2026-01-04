use memo_with_references::memo;

#[memo('a, 'b)]
fn test_multiple_lifetimes() {}

fn main() {}
