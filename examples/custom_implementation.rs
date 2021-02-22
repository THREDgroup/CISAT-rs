use cisat::{AgentMethods, Cohort, Parameters, Solution, TeamMethods};
use std::cmp::{Eq, Ord, Ordering, PartialOrd};
use std::ops::Sub;

#[derive(Debug, Clone)]
struct CustomProblem {}

impl Solution for CustomProblem {
    const NUMBER_OF_MOVE_OPERATORS: usize = 0;
    const NUMBER_OF_OBJECTIVES: usize = 0;

    fn new() -> Self {
        unimplemented!()
    }

    fn apply_move_operator(&mut self, move_index: usize, temperature: f64) {
        unimplemented!()
    }

    fn get_quality_scalar(&self) -> f64 {
        unimplemented!()
    }
}

impl Sub for CustomProblem {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl PartialEq for CustomProblem {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

impl PartialOrd for CustomProblem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}

impl Eq for CustomProblem {}

impl Ord for CustomProblem {
    fn cmp(&self, other: &Self) -> Ordering {
        unimplemented!()
    }
}

struct CustomAgent {
    current_solution: CustomProblem,
    best_solution_so_far: CustomProblem,
}

impl AgentMethods<CustomProblem> for CustomAgent {
    fn new(id: usize, parameters: Parameters) -> Self {
        unimplemented!()
    }

    fn iterate(&mut self) {
        unimplemented!()
    }

    fn get_best_solution_so_far(&mut self) -> CustomProblem {
        unimplemented!()
    }

    fn get_current_solution(&mut self) -> CustomProblem {
        unimplemented!()
    }

    fn communicate(&mut self, solutions: Vec<CustomProblem>) {
        unimplemented!()
    }
}

struct CustomTeam {}

impl TeamMethods<CustomProblem, CustomAgent> for CustomTeam {
    fn new(parameters: Parameters) -> Self {
        unimplemented!()
    }

    fn iterate(&mut self) {
        unimplemented!()
    }

    fn communicate(&mut self) {
        unimplemented!()
    }

    fn solve(&mut self) {
        unimplemented!()
    }

    fn get_best_solution_so_far(&mut self) -> CustomProblem {
        unimplemented!()
    }
}

fn main() {
    let mut x = Cohort::<CustomProblem, CustomAgent, CustomTeam>::new(Parameters::default());
    x.solve();
}
