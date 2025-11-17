use flags::Flags;
use possible_dimensions::PossibleDimensions;

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

    pub fn varies_by(&self, dimension_name: &str) -> bool {
        let idx = self.dims.index_of(dimension_name);
        self.flags.varies_by(idx)
    }

    pub fn run_length(&self, dimension_name: &str) -> &usize {
        let idx = self.dims.index_of(dimension_name);
        self.flags.run_length(idx)
    }
}
