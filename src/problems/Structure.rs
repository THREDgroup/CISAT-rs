use super::super::utilities::Solution;

#[derive(Clone, Debug)]
pub struct Structure {
    quality: Vec<f64>,
}

impl Solution<Structure> for Structure {
    const NUMBER_OF_MOVE_OPERATORS: usize = 1;
    const NUMBER_OF_OBJECTIVES: usize = 1;
    fn generate_initial_solution() -> Structure {
        Structure {
            quality: vec![0.0; Structure::NUMBER_OF_OBJECTIVES],
        }
    }

    fn apply_move_operator(&mut self, move_index: usize) {}

    fn get_quality(&mut self) -> Vec<f64> {
        self.quality.to_vec()
    }
}
