use memo_with_references::memo;

#[memo('a)]
fn memome<'a>(data: &'a mut String) {}

fn main() {}
