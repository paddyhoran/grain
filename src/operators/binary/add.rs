use crate::Data;

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
    todo!()
}
