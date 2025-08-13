use memo_with_references::memo;

struct Foo;

impl Foo {
    #[memo(cache)]
    fn memome<'a>(&mut self, cache: &'a str) {}
}

fn main() {}
