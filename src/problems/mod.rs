//! This module contains some pre-implemented problems for demonstration and testing purposes. To
//! implement your own problems, simply create a class that implements the ```Solution``` trait.

// pub mod structure;
// pub use structure::Structure;

/// This re-exports the Ackley problem
pub mod ackley;
pub use ackley::Ackley;
