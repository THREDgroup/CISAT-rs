//! This is an example problem for designing truss structures
use super::super::utilities::Solution;
use crate::utilities::randomness::multinomial_tuple_draw;
use std::{cmp::Ordering, ops::Sub};
use trussx::{StructuralShape, Truss};

#[derive(Clone, Debug)]
/// This is a structure!
pub struct Structure {
    /// This contains the fundamental truss information
    truss: Truss,
    /// This contains a single quality scalar derived from objective function values
    quality_scalar: f64,
    /// This contains direct objective function values
    objective_function_value: Vec<f64>,
}

impl Solution for Structure {
    const NUMBER_OF_MOVE_OPERATORS: usize = 1;
    const NUMBER_OF_OBJECTIVES: usize = 1;
    fn new() -> Structure {
        // Create a seed truss
        let mut x = Truss::new();
        let a = x.add_joint([-5.0, 0.0, 0.0]);
        let b = x.add_joint([-2.0, 0.0, 0.0]);
        let c = x.add_joint([1.0, 0.0, 0.0]);
        let d = x.add_joint([3.0, 0.0, 0.0]);
        let c = x.add_joint([5.0, 0.0, 0.0]);
        // TODO: Add more nodes, connect them, set reactions, set forces

        // Return the new truss
        Structure {
            truss: x,
            quality_scalar: 0.0,
            objective_function_value: vec![0.0; Structure::NUMBER_OF_OBJECTIVES],
        }
    }

    fn apply_move_operator(&mut self, move_index: usize, _temperature: f64) {
        match move_index {
            0 => self.add_joint_and_attach(),
            1 => self.remove_joint(),
            2 => self.add_member(),
            3 => self.remove_member(),
            4 => self.change_size_single(),
            5 => self.change_size_all(),
            6 => self.move_joint(),
            _ => panic!(
                "The move index {} is not in the valid range [0, {}]",
                move_index,
                Structure::NUMBER_OF_MOVE_OPERATORS
            ),
        }
    }

    fn get_quality_scalar(&self) -> f64 {
        self.quality_scalar
    }
}

impl Structure {
    /// Add a joint adn attach it
    fn add_joint_and_attach(&mut self) {
        unimplemented!()
    }
    /// Remove a joint
    fn remove_joint(&mut self) {
        unimplemented!()
    }
    /// Add a member
    fn add_member(&mut self) {}

    /// Remove a member
    fn remove_member(&mut self) {
        let fos = self.truss.get_fos();
        let idx = multinomial_tuple_draw(fos);
        self.truss.delete_member(idx);
    }
    /// Change the size of a single member
    fn change_size_single(&mut self) {
        unimplemented!()
    }
    /// Change the size of all members
    fn change_size_all(&mut self) {
        unimplemented!()
    }
    /// Move a joint
    fn move_joint(&mut self) {
        unimplemented!()
    }
}

impl PartialEq for Structure {
    fn eq(&self, other: &Self) -> bool {
        self.quality_scalar == other.quality_scalar
    }
}

impl Eq for Structure {}

impl PartialOrd for Structure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quality_scalar.partial_cmp(&other.quality_scalar)
    }
}

impl Ord for Structure {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for Structure {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.quality_scalar - rhs.quality_scalar
    }
}
