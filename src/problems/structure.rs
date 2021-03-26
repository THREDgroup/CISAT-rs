//! This is an example problem for designing truss structures
use super::super::utilities::Solution;
use crate::utilities::randomness::multinomial_tuple_draw;
use std::{cmp::Ordering, ops::Sub};
use trussx::{StructuralShape, Truss};

/// Member radius and wall thickness
const MEMBER_RADIUS: [f64; 10] = [
    0.005, 0.010, 0.015, 0.020, 0.025, 0.030, 0.035, 0.040, 0.045, 0.050,
];

/// Radius thickness ratio for pipes
const RADIUS_THICKNESS_RATIO: f64 = 7.5;

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
    const NUMBER_OF_MOVE_OPERATORS: usize = 7;
    const NUMBER_OF_OBJECTIVES: usize = 1;
    fn new() -> Structure {
        // Create a seed truss
        let mut jidx = vec![];
        let mut eidx = vec![];
        let mut x = Truss::new();

        // Create truss nodes
        jidx.push(x.add_joint([-5.0, 0.0, 0.0]));
        x.set_reactions(jidx[0], [true, true, true]);
        jidx.push(x.add_joint([-2.0, 0.0, 0.0]));
        x.set_reactions(jidx[1], [true, true, true]);
        x.set_loads(jidx[1], [0.0, -20_000.0, 0.0]);
        jidx.push(x.add_joint([1.0, 0.0, 0.0]));
        x.set_reactions(jidx[2], [true, true, true]);
        jidx.push(x.add_joint([3.0, 0.0, 0.0]));
        x.set_reactions(jidx[3], [true, true, true]);
        x.set_loads(jidx[3], [0.0, -20_000.0, 0.0]);
        jidx.push(x.add_joint([5.0, 0.0, 0.0]));
        x.set_reactions(jidx[4], [true, true, true]);

        // Create truss members
        eidx.push(x.add_edge(jidx[0], jidx[1]));
        x.set_shape_for_all(StructuralShape::Pipe {
            outer_radius: MEMBER_RADIUS[4],
            thickness: MEMBER_RADIUS[4] / RADIUS_THICKNESS_RATIO,
            center_of_gravity: (0.0, 0.0),
        });
        x.set_material_for_all(209_000_000_000.00, 344_000_000.0);
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
        let fos = self.truss.get_fos_tuple();
        let idx = multinomial_tuple_draw(fos);
        self.truss.delete_member(idx);
    }
    /// Change the size of a single member
    fn change_size_single(&mut self) {
        unimplemented!()
    }

    /// Change the size of all members
    fn change_size_all(&mut self) {
        let fos = self.truss.get_fos();
        // self.truss.set_shape_for_all();
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
