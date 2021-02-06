use super::super::{
    utilities::randomness::{random_gaussian_vector, random_uniform_vector},
    utilities::Solution,
};
use std::{cmp::Ordering, ops::Sub};

const NUMBER_OF_DIMENSIONS: usize = 5;

#[derive(Clone, Debug)]
/// This contains solutions for the Ackley problem
pub struct Ackley {
    /// This contains direct objective function values
    objective_function_value: Vec<f64>,
    /// This contains a single quality scalar derived from objective function values
    quality_scalar: f64,
    /// This contains the parameters
    x: Vec<f64>,
}

impl Solution<Ackley> for Ackley {
    const NUMBER_OF_MOVE_OPERATORS: usize = 1;
    const NUMBER_OF_OBJECTIVES: usize = 1;

    fn generate_initial_solution() -> Ackley {
        let mut solution = Ackley {
            objective_function_value: vec![0.0; Ackley::NUMBER_OF_OBJECTIVES],
            x: random_uniform_vector(NUMBER_OF_DIMENSIONS, -5.0, 5.0),
            quality_scalar: 0.0,
        };
        solution.evaluate();
        solution
    }

    fn apply_move_operator(&mut self, _move_index: usize, temperature: f64) {
        let perturbation = random_gaussian_vector(self.x.len(), 0.0, temperature);
        for i in 0..self.x.len() {
            self.x[i] += perturbation[i];
        }
        self.evaluate();
    }

    fn get_quality_scalar(&self) -> f64 {
        self.quality_scalar
    }
}

impl Ackley {
    /// This function offers some functionality for evaluation
    fn evaluate(&mut self) {
        let n = self.x.len();
        let mut fx = 0.0;
        let mut square_sum = 0.0;
        let mut cosine_sum = 0.0;
        for xi in self.x.to_vec() {
            square_sum += xi.powi(2);
            cosine_sum += (2.0 * std::f64::consts::PI * xi).cos();
        }
        fx += -20.0 * (-0.2 * (0.5 * square_sum).sqrt()).exp();
        fx -= (cosine_sum / (n as f64)).exp();
        fx += std::f64::consts::E + 20.0;
        self.objective_function_value = vec![fx; 1];
        self.quality_scalar = -fx;
    }
}

impl PartialEq for Ackley {
    fn eq(&self, other: &Self) -> bool {
        self.quality_scalar == other.quality_scalar
    }
}

impl Eq for Ackley {}

impl PartialOrd for Ackley {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quality_scalar.partial_cmp(&other.quality_scalar)
    }
}

impl Ord for Ackley {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for Ackley {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.quality_scalar - rhs.quality_scalar
    }
}
