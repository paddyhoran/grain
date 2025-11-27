//! Contains the implementation of the `PossibleDimensions` type.
//!
//! This type is used to track the current dimensions that are possible
//! based on the data that has been encountered.
//!
//! How dimensions are ordered is encoded in this type.  Currently,
//! higher cardinality dimensions are push toward the right which should
//! allow zero copy slicing larger regions of data.

use indexmap::IndexMap;

/// Holds the actual values that are possible within a dimension.
#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct DimensionValues(Vec<String>);

/// The collection of all possible dimensions that a value **could** vary by.
#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub struct PossibleDimensions(IndexMap<String, DimensionValues>);

impl PossibleDimensions {
    /// Returns the index of the dimension with name `dimension_name`.
    pub fn index_of(&self, dimension_name: &str) -> usize {
        self.0
            .get_index_of(dimension_name)
            .unwrap_or_else(|| panic!("Un-recognised dimension: '{}'", dimension_name))
    }

    pub fn index_of_value(&self, dim_index: usize, value: &String) -> usize {
        let values = &self.0[dim_index];
        values.0.iter().position(|s| s == value).unwrap()
    }

    /// Returns the index of the dimension with name `dimension_name`.
    pub fn maybe_index_of(&self, dimension_name: &str) -> Option<usize> {
        self.0.get_index_of(dimension_name)
    }

    /// Builder type API for adding new dimensions.
    pub fn add_dimension(mut self, name: String, values: Vec<String>) -> Self {
        self.0.insert(name, DimensionValues(values));
        self
    }

    pub fn sizes(&self) -> Vec<usize> {
        self.0.values().map(|v| v.0.len()).collect()
    }
}

/// Combines two instances of `PossibleDimensions` creating a new `PossibleDimenions` that
/// contains all the dimensions of `lhs` and `lhs`.
#[allow(dead_code)]
pub(crate) fn combine_dimensions(
    lhs: &PossibleDimensions,
    rhs: &PossibleDimensions,
) -> PossibleDimensions {
    let mut new_possible_dimensions = IndexMap::new();

    let mut lhs_iter = lhs.0.iter();
    let mut rhs_iter = rhs.0.iter();

    let mut l = lhs_iter.next();
    let mut r = rhs_iter.next();

    loop {
        if l.is_none() {
            l = lhs_iter.next();
        }
        if r.is_none() {
            r = rhs_iter.next();
        }

        match (l, r) {
            (None, None) => break,
            (None, Some((name, values))) => {
                let _ = new_possible_dimensions.insert(name.clone(), values.clone());
                for (k, v) in rhs_iter {
                    let _ = new_possible_dimensions.insert(k.clone(), v.clone());
                }
                break;
            }
            (Some((name, values)), None) => {
                let _ = new_possible_dimensions.insert(name.clone(), values.clone());
                for (k, v) in rhs_iter {
                    let _ = new_possible_dimensions.insert(k.clone(), v.clone());
                }
                break;
            }
            (Some((l_name, l_values)), Some((r_name, r_values))) => {
                if l_name == r_name {
                    if l_values == r_values {
                        new_possible_dimensions.insert(l_name.clone(), l_values.clone());
                        l = None;
                        r = None;
                    } else {
                        panic!("Dimension '{}' has conflicting values.", l_name)
                    }
                } else {
                    if l_values.0.len() == r_values.0.len() {
                        if l_name <= r_name {
                            new_possible_dimensions.insert(l_name.clone(), l_values.clone());
                            l = None;
                        } else {
                            new_possible_dimensions.insert(r_name.clone(), r_values.clone());
                            r = None;
                        }
                    } else if l_values.0.len() < r_values.0.len() {
                        new_possible_dimensions.insert(l_name.clone(), l_values.clone());
                        l = None;
                    } else {
                        new_possible_dimensions.insert(r_name.clone(), r_values.clone());
                        r = None;
                    }
                }
            }
        }
    }

    PossibleDimensions(new_possible_dimensions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_single_dimension() {
        let a = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "b".to_string()]);
        let b = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "b".to_string()]);

        let c = combine_dimensions(&a, &b);

        assert_eq!(c.0.len(), 1);
        assert_eq!(c, a);
    }

    #[test]
    fn test_same_multiple_dimension() {
        let a = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "b".to_string()])
            .add_dimension("2".to_string(), vec!["a".to_string(), "b".to_string()]);
        let b = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "b".to_string()])
            .add_dimension("2".to_string(), vec!["a".to_string(), "b".to_string()]);

        let c = combine_dimensions(&a, &b);

        assert_eq!(c.0.len(), 2);
        assert_eq!(c, a);
    }

    #[test]
    #[should_panic]
    fn test_values_differ() {
        let a = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "b".to_string()]);
        let b = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "c".to_string()]);
        let _ = combine_dimensions(&a, &b);
    }

    #[test]
    fn test_expand() {
        let a = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "b".to_string()])
            .add_dimension("2".to_string(), vec!["c".to_string(), "d".to_string()]);
        let b = PossibleDimensions::default()
            .add_dimension("1".to_string(), vec!["a".to_string(), "b".to_string()])
            .add_dimension("3".to_string(), vec!["e".to_string(), "f".to_string()]);

        let c = combine_dimensions(&a, &b);
        assert_eq!(c.0.len(), 3);

        // First dimension
        let (key, values) = c.0.get_index(0).unwrap();
        assert_eq!(key, "1");
        assert_eq!(values.0, vec!["a", "b"]);

        // Second dimension
        let (key, values) = c.0.get_index(1).unwrap();
        assert_eq!(key, "2");
        assert_eq!(values.0, vec!["c", "d"]);

        // Third dimension
        let (key, values) = c.0.get_index(2).unwrap();
        assert_eq!(key, "3");
        assert_eq!(values.0, vec!["e", "f"]);
    }

    #[test]
    fn test_sort_by_cardinality() {
        let a = PossibleDimensions::default()
            .add_dimension("2".to_string(), vec!["c".to_string(), "d".to_string()])
            .add_dimension(
                "1".to_string(),
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
            );
        let b = PossibleDimensions::default()
            .add_dimension("3".to_string(), vec!["e".to_string(), "f".to_string()]);

        let c = combine_dimensions(&a, &b);
        dbg!(&c);
        assert_eq!(c.0.len(), 3);

        // First dimension
        let (key, values) = c.0.get_index(0).unwrap();
        assert_eq!(key, "2");
        assert_eq!(values.0, vec!["c", "d"]);

        // Second dimension
        let (key, values) = c.0.get_index(1).unwrap();
        assert_eq!(key, "3");
        assert_eq!(values.0, vec!["e", "f"]);

        // Third dimension
        let (key, values) = c.0.get_index(2).unwrap();
        assert_eq!(key, "1");
        assert_eq!(values.0, vec!["a", "b", "c"]);
    }
}
