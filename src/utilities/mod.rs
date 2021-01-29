pub mod parameters;

pub trait Solution<T> {
    const NUMBER_OF_MOVE_OPERATORS: usize;
    const NUMBER_OF_OBJECTIVES: usize;
    fn generate_initial_solution() -> T;
    fn apply_move_operator(&mut self, move_index: usize);
    fn get_quality(&mut self) -> Vec<f64>;
}
