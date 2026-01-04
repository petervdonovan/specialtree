use memo_with_references::memo;

struct Foo;

impl Foo {
    #[memo('a)]
    fn memome<'a>(&mut self) {}
}

fn main() {}
