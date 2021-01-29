use super::super::utilities::Solution;

#[derive(Clone)]
pub struct Example {
    quality: Vec<f64>,
}

impl Solution<Example> for Example {
    const NUMBER_OF_MOVE_OPERATORS: usize = 1;
    const NUMBER_OF_OBJECTIVES: usize = 1;

    fn generate_initial_solution() -> Example {
        Example {
            quality: vec![0.0; Example::NUMBER_OF_OBJECTIVES],
        }
    }

    fn apply_move_operator(&mut self, move_index: usize) {}

    fn get_quality(&mut self) -> Vec<f64> {
        self.quality.to_vec()
    }
}
