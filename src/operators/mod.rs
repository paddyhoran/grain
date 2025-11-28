mod mul;

pub use mul::*;
use arrow_array::Array;

use crate::data::Values;
use arrow_buffer::Buffer;

/// Performs a scalar binary operation on `values`.
///
/// Note, this function assumes that `values` does not have an allocated bitmap.
fn scalar_binary_op<F>(values: &Values, scalar: f64, op: F) -> Values
where
    F: Fn(f64, f64) -> f64,
{
    debug_assert!(values.null_count() == 0);
    //  Soundness: `values` is an iterator with a known size from a PrimitiveArray
    let buffer =
        unsafe { Buffer::from_trusted_len_iter(values.values().iter().map(|l| op(*l, scalar))) };
    Values::new(buffer.into(), None)
}

/// Performs binary operation between two `Values`.
///
/// Note, this function assumes that both `Values`'s are the same size and neither has an allocated bitmap.

fn array_binary_op<F>(lhs: &Values, rhs: &Values, op: F) -> Values
where
    F: Fn(f64, f64) -> f64,
{
    debug_assert!(
        lhs.len() == rhs.len(),
        "a ({}) and b ({}) have different lengths.",
        lhs.len(),
        rhs.len()
    );
    //  Soundness: `values` is an iterator with a known size from a PrimitiveArray
    let buffer = unsafe {
        Buffer::from_trusted_len_iter(
            lhs.values()
                .iter()
                .zip(rhs.values())
                .map(|(l, r)| op(*l, *r)),
        )
    };
    Values::new(buffer.into(), None)
}
