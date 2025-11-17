mod add;

pub use add::*;

use crate::data::Values;
use arrow_buffer::Buffer;

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
