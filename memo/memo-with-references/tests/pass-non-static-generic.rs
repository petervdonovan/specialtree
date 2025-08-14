use memo_with_references::memo;

#[memo('a)]
fn non_static_generic<'a, T>(x: &'a T) -> &'a T {
    x
}

fn main() {}
