mod errors;
mod hex_math;
mod numgen;
mod utils;

use numgen::BeamPathGenerator;

pub use numgen::Bounds;

pub struct GeneratedNumber {
    pub direction: String,
    pub pattern: String,
    pub largest_dimension: u32,
    pub num_points: usize,
}

pub fn generate_number_pattern_beam(
    target: i32,
    bounds: Bounds,
    carryover: usize,
    trim_larger: bool,
) -> Option<GeneratedNumber> {
    let path = BeamPathGenerator::new(target, bounds, carryover, trim_larger).run()?;
    Some(GeneratedNumber {
        direction: path.starting_direction().to_string(),
        pattern: path.pattern(),
        largest_dimension: path.bounds().largest_dimension(),
        num_points: path.num_points(),
    })
}
