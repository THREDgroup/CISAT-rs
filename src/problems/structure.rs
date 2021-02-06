//! This is an example problem for designing truss structures
use super::super::utilities::Solution;
use std::{cmp::Ordering, ops::Sub};
use trussx::Truss;

#[derive(Clone, Debug)]
/// This is a structure!
pub struct Structure {
    /// Thie contains the fundamental truss information
    truss: Truss,
    /// This contains a single quality scalar derived from objective function values
    quality_scalar: f64,
    /// This contains direct objective function values
    objective_function_value: Vec<f64>,
}

impl Solution<Structure> for Structure {
    const NUMBER_OF_MOVE_OPERATORS: usize = 1;
    const NUMBER_OF_OBJECTIVES: usize = 1;
    fn generate_initial_solution() -> Structure {
        Structure {
            truss: Truss::new(),
            quality_scalar: 0.0,
            objective_function_value: vec![0.0; Structure::NUMBER_OF_OBJECTIVES],
        }
    }

    fn apply_move_operator(&mut self, move_index: usize, temperature: f64) {
        unimplemented!()
    }

    fn get_quality_scalar(&self) -> f64 {
        unimplemented!()
    }
}

impl PartialEq for Structure {
    fn eq(&self, other: &Self) -> bool {
        self.quality_scalar == other.quality_scalar
    }
}

impl Eq for Structure {}

impl PartialOrd for Structure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quality_scalar.partial_cmp(&other.quality_scalar)
    }
}

impl Ord for Structure {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for Structure {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.quality_scalar - rhs.quality_scalar
    }
}
