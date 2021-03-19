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

    // Replace elements with uniform normals
    for elem in random_vector.iter_mut().take(length) {
        *elem += normal.sample(&mut rng);
    }

    random_vector
}

/// This make a multinomial draw from a set of weights - think a loaded die
pub(crate) fn multinomial_draw(weights: Vec<f64>) -> usize {
    let mut rng = thread_rng();
    let weighted = WeightedIndex::new(weights).unwrap();
    weighted.sample(&mut rng)
}

/// This make a multinomial draw from a set of weights with identifiers - think a loaded die with weird names for the faces
pub(crate) fn multinomial_tuple_draw<T: Copy>(identifiers_and_weights: Vec<(T, f64)>) -> T {
    // Split weights and indices
    let mut weights = vec![];
    let mut identifiers = vec![];
    for elem in identifiers_and_weights.into_iter() {
        weights.push(elem.1);
        identifiers.push(elem.0);
    }

    // Make rng thread
    let mut rng = thread_rng();

    // Perform weighted draw and get index
    let weighted = WeightedIndex::new(weights).unwrap();
    let idx = weighted.sample(&mut rng);
    identifiers[idx]
}

/// This generates a random vector between uniform bounds
pub(crate) fn random_uniform_vector(length: usize, low: f64, high: f64) -> Vec<f64> {
    // Make a blank random vector
    let mut random_vector = vec![0.0; length];

    // Make a distribution to upll from
    let mut rng = thread_rng();
    let uniform = Uniform::new_inclusive(low, high);

    // Replace elemnts with uniform normals
    for elem in random_vector.iter_mut().take(length) {
        *elem += uniform.sample(&mut rng);
    }

    random_vector
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
    use crate::utilities::randomness::{
        multinomial_draw, random_gaussian_vector, random_uniform_vector,
    };

    #[test]
    fn test_gaussian_vector_generation() {
        let x = random_gaussian_vector(5, 0.0, 1.0);
        println!("gaussian: {:?}", x);
    }

    #[test]
    fn test_uniform_vector_generation() {
        let x = random_uniform_vector(5, 0.0, 1.0);
        println!("uniform: {:?}", x);
    }

    #[test]
    fn test_multinomial_draw() {
        let x = multinomial_draw(vec![1.0, 2.3, 10.1, 3.1]);
        println!("mult: {:?}", x);
    }
}
