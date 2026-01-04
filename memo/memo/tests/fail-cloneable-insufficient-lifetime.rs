use memo_with_references::memo;

struct DataWithLifetime<'b> {
    data: &'b str,
}

// This should fail because DataWithLifetime<'b> cannot outlive 'a
// when 'b is not bound to outlive 'a
#[memo('a)]
fn test_insufficient_lifetime<'a, 'b>(x: DataWithLifetime<'b>) -> &'a DataWithLifetime<'b> {
    unsafe { std::mem::transmute(&x) }
}

fn main() {}
