use super::super::utilities::Solution;

#[derive(Clone, Debug)]
pub struct Structure {
    objective_function_value: Vec<f64>,
}

impl Solution<Structure> for Structure {
    const NUMBER_OF_MOVE_OPERATORS: usize = 1;
    const NUMBER_OF_OBJECTIVES: usize = 1;
    fn generate_initial_solution() -> Structure {
        Structure {
            objective_function_value: vec![0.0; Structure::NUMBER_OF_OBJECTIVES],
        }
    }

    fn apply_move_operator(&mut self, move_index: usize, temperature: f64) {}

    fn get_quality_scalar(&mut self) -> f64 {
        unimplemented!()
    }
}
