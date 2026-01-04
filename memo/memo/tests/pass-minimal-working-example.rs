use memo_with_references::memo;

#[memo('a)]
fn memome<'a>() -> &'a i32 {
    &42
}

fn main() {}
