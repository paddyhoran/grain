pub struct Query {
    pub dimension_name: String,
    pub dimension_value: String,
}

#[cfg(test)]
mod tests {
    use crate::Data;

    use super::*;

    #[test]
    fn test_single_dimension() {
        let data = Data::new_from_iter(
            "test".to_string(),
            [("A".to_string(), 1.0), ("B".to_string(), 2.0)].into_iter(),
        );

        let query = Query {
            dimension_name: "test".to_string(),
            dimension_value: "B".to_string(),
        };

        let data = data.query(&query);
        let values = data.values;
        assert_eq!(values.len(), 1);
        assert_eq!(values.value(0), 2.0);
    }
}
