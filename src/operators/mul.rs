use crate::Data;

use super::{array_binary_op, scalar_binary_op};

/// Performs a multiplication operation (*) expanding the granularity of
/// either operand as required.
///
/// This is often called "broadcasting".  Whether it is correct to broadcast
/// depends on what the data represents.
pub fn mul(lhs: &Data, rhs: &Data) -> Data {
    todo!()
}

/// Performs a muliplication operation (*) but only if the level of
/// granularity is the same.
///
/// # Panics
///
/// If the granularity of the two operands is not the same.
pub fn mul_strict(lhs: &Data, rhs: &Data) -> Data {
    if lhs.granularity() != rhs.granularity() {
        panic!(
            "When using the strict version of operators (mul in
            this case) the granularity must match."
        )
    }

    let values = array_binary_op(lhs.values(), rhs.values(), |a, b| a * b);
    Data {
        granularity: lhs.granularity().clone(),
        values,
    }
}

/// Adds a scalar `amount` to `data`.
pub fn mul_scalar(data: &Data, amount: f64) -> Data {
    let values = scalar_binary_op(data.values(), amount, |a, b| a * b);
    Data {
        granularity: data.granularity().clone(),
        values,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_strict() {
        let data_1 = Data::new_from_iter("test".to_string(), [("A".to_string(), 3.0)].into_iter());
        let data_2 = Data::new_from_iter("test".to_string(), [("A".to_string(), 5.0)].into_iter());
        let data_3 = mul_strict(&data_1, &data_2);

        let values = data_3.values();
        assert_eq!(values.len(), 1);
        let value = values.value(0);
        assert_eq!(value, 15.0);
    }

    #[test]
    fn test_mul_scalar() {
        let data_1 = Data::new_from_iter("test".to_string(), [("A".to_string(), 1.0)].into_iter());
        let data_2 = mul_scalar(&data_1, 3.0);

        let values = data_2.values();
        assert_eq!(values.len(), 1);
        let value = values.value(0);
        assert_eq!(value, 3.0);
    }
}
