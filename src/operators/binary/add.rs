use crate::Data;

use super::{array_binary_op, scalar_binary_op};

/// Performs a add operation (+) expanding the granularity of either
/// operand as required.
///
/// This is often called "broadcasting" in other contexts.
pub fn add(lhs: &Data, rhs: &Data) -> Data {
    todo!()
}

/// Performs a add operation (+) but only if the level of granularity
/// is the same.
///
/// # Panics
///
/// If the granularity of the two operands is not the same.
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


/// Adds a scalar `amount` to `data`. 
pub fn add_scalar(data: &Data, amount: f64) -> Data {

    let values = scalar_binary_op(data.values(), amount, |a, b| a + b);
    Data {
        granularity: data.granularity().clone(),
        values,
    }
}
