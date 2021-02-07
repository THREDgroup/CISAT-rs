//! This contains some randomization functions that are useful

use rand::thread_rng;
use rand_distr::{Distribution, Normal, Uniform, WeightedIndex};

/// This is a function for generating a random vector from a Gaussian distribution
pub(crate) fn random_gaussian_vector(
    length: usize,
    mean: f64,
    standard_deviation: f64,
) -> Vec<f64> {
    // Make a blank random vector
    let mut random_vector = vec![0.0; length];

    // Make a distribution to upll from
    let mut rng = thread_rng();
    let normal = Normal::new(mean, standard_deviation).unwrap();

    // Replace elemnts with uniform normals
    for i in 0..length {
        let dd = normal.sample(&mut rng);
        random_vector[i] += dd;
    }

    random_vector
}

/// This make a multinomial draw from a set of weights - think a loaded die
pub(crate) fn multinomial_draw(weights: Vec<f64>) -> usize {
    let mut rng = thread_rng();
    let weighted = WeightedIndex::new(weights).unwrap();
    weighted.sample(&mut rng)
}

/// This generates a random vector between uniform bounds
pub(crate) fn random_uniform_vector(length: usize, low: f64, high: f64) -> Vec<f64> {
    // Make a blank random vector
    let mut random_vector = vec![0.0; length];

    // Make a distribution to upll from
    let mut rng = thread_rng();
    let uniform = Uniform::new_inclusive(low, high);

    // Replace elemnts with uniform normals
    for i in 0..length {
        random_vector[i] += uniform.sample(&mut rng);
    }

    return random_vector;
}

/// Random number between 0 and 1

pub(crate) fn random_unit_draw() -> f64 {
    // Make a distribution to upll from
    let mut rng = thread_rng();
    let uniform = Uniform::new_inclusive(0.0, 1.0);
    uniform.sample(&mut rng)
}

#[cfg(test)]
mod random_tests {
    use crate::utilities::randomness::{multinomial_draw, random_gaussian_vector};

    #[test]
    fn test_vector_generation() {
        let x = random_gaussian_vector(5, 0.0, 1.0);
    }

    #[test]
    fn test_multinomial_draw() {
        let x = multinomial_draw(vec![1.0, 2.3, 10.1, 3.1]);
    }
}
