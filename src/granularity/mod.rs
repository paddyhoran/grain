use flags::Flags;
use possible_dimensions::PossibleDimensions;

use crate::query::Query;

mod flags;
mod possible_dimensions;

/// Holds meta-data that allows the actual data
/// array to be interpreted.
#[derive(PartialEq, Eq, Clone)]
pub struct Granularity {
    /// The dimensions that the data actually "varies by".
    flags: Flags,

    /// The current possible dimensions.
    dims: PossibleDimensions,
}

impl Granularity {
    pub fn new(dimension_name: String, dimension_values: Vec<String>) -> Self {
        Self {
            flags: Default::default(),
            dims: PossibleDimensions::default().add_dimension(dimension_name, dimension_values),
        }
    }

    pub fn size(&self) -> usize {
        self.flags.size()
    }

    pub fn flags(&self) -> &Flags {
        &self.flags
    }

    pub fn varies_by(&self, dimension_name: &str) -> bool {
        let idx = self.dims.index_of(dimension_name);
        self.flags.varies_by(idx)
    }

    /// Returns the index of a specific dimension called `dimension_name`.
    ///
    /// If `dimension_name` does not existi
    pub fn dimension_index(&self, dimension_name: &str) -> Option<usize> {
        self.dims.maybe_index_of(dimension_name)
    }

    pub fn run_length(&self, dimension_name: &str) -> &usize {
        let idx = self.dims.index_of(dimension_name);
        self.flags.run_length(idx)
    }

    pub fn broadcast(&self, other: &Self) -> Self {
        if self.dims == other.dims {
            let flags = self.flags.broadcast(&other.flags, &self.dims.sizes());
            Self {
                flags,
                dims: self.dims.clone(),
            }
        } else {
            todo!()
        }
    }

    pub fn drop(&mut self, dimension_name: &str) {
        let idx = self.dims.index_of(dimension_name);
        self.flags.drop(idx);
    }

    /// Calculates the offsets into each dimensions possible values required
    /// by `query`.
    ///
    /// A `None` value indicates that the dimension is not needed for `query`.
    fn dimension_offsets(&self, query: &Query) -> Vec<Option<usize>> {
        let mut values: Vec<_> = (0..self.size()).map(|_| None).collect();
        let dimension_index = self.dimension_index(&query.dimension_name);

        if let Some(idx) = dimension_index {
            let flags = self.flags();
            if flags.varies_by(idx) {
                values[idx] = Some(self.dims.index_of_value(idx, &query.dimension_value));
            }
        }
        values
    }

    /// Calculates the offset into the `values` array needed to access
    /// the result of `query`.
    pub fn data_offset(&self, query: &Query) -> usize {
        let dimension_offsets = self.dimension_offsets(query);
        self.flags
            .run_lengths()
            .iter()
            .zip(
                dimension_offsets
                    .iter()
                    .map(|offset| offset.unwrap_or_default()),
            )
            .map(|(run_length, index)| run_length * index)
            .reduce(|a, b| a + b)
            .unwrap_or(0)
    }
}
