use crate::{Data, operators::binary::array_binary_op};

/// Performs a add operation (+) expanding the granularity of either
/// operand as required.
///
/// This is often called "broadcasting" in other contexts.
pub fn add(lhs: &Data, rhs: &Data) -> Data {
    todo!()
}

/// Performs a add operation (+) but only if the level of granularity
/// is the same.  If this is not the case an error is returned.
pub fn add_strict(lhs: &Data, rhs: &Data) -> Data {
    if lhs.granularity() != rhs.granularity() {
        panic!(
            "When using the strict version of operators (add in
            this case) the granularity must match."
        )
    }

    let values = array_binary_op(lhs.values(), rhs.values(), |a, b| a + b);
    Data {
        granularity: lhs.granularity().clone(),
        values,
    }
}
