use memo_with_references::memo;

struct Foo;

impl Foo {
    #[memo(self)]
    fn memome(&self) {}
}

fn main() {}
