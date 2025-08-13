use memo_with_references::memo;

#[memo('a)]
fn test_working_mixed_lifetimes<'a, 'b>(
    cache_data: &'a String, // frozen (matches cache lifetime)
    other_data: &'b String, // cloneable (different lifetime, String implements Clone)
    owned_data: String,     // cloneable (owned, String implements Clone)
) -> &'a String {
    cache_data
}

fn main() {}
