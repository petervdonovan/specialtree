use memo_with_references::memo;

#[memo('a)]
fn test_unit_return() -> () {
    println!("This has side effects and shouldn't be memoized");
}

fn main() {}
