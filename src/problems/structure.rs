use super::super::utilities::Solution;
use trussx;

#[derive(Clone, Debug)]
pub struct Structure {
    truss: trussx::Truss,
    objective_function_value: Vec<f64>,
}

impl Solution<Structure> for Structure {
    const NUMBER_OF_MOVE_OPERATORS: usize = 1;
    const NUMBER_OF_OBJECTIVES: usize = 1;
    fn generate_initial_solution() -> Structure {
        Structure {
            truss: trussx::Truss::new(),
            objective_function_value: vec![0.0; Structure::NUMBER_OF_OBJECTIVES],
        }
    }

    fn apply_move_operator(&mut self, move_index: usize, temperature: f64) {}
}
