use super::super::utilities::Solution;

#[derive(Clone)]
pub struct Truss {
    x: f64,
}

impl Solution<Truss> for Truss {
    fn generate_solution() -> Truss {
        Truss { x: 1.0 }
    }
}
