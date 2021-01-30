use std::ops::Sub;

pub trait Solution<T>: PartialOrd + Sub<Output = f64> + Sized {
    const NUMBER_OF_MOVE_OPERATORS: usize;
    const NUMBER_OF_OBJECTIVES: usize;
    fn generate_initial_solution() -> T;
    fn apply_move_operator(&mut self, move_index: usize, temperature: f64);
}
