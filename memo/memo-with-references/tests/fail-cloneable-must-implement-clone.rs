use memo_with_references::memo;

// This should fail because NoClone doesn't implement Clone
// but it's a cloneable argument (not a reference with cache lifetime)
struct NoClone;

#[memo('a)]
fn test_cloneable_must_implement_clone<'a, 'b>(
    data: &'a String,  // frozen (matches cache lifetime)
    other: &'b String, // cloneable (different lifetime) - should require Clone
    owned: NoClone,    // cloneable (owned) - should require Clone
) -> &'a String {
    data
}

fn main() {}
