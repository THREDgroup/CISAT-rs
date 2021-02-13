#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]

//! This crate contains an implementation of the Cognitively-Inspired Simulated Annealing Teams \
//! (CISAT) framework.

mod utilities;
pub use utilities::{
    parameters::{CommunicationStyle, OperationalLearning, Parameters, TemperatureSchedule},
    Solution,
};

mod abm;
pub use abm::agent::Agent;
pub use abm::cohort::Cohort;
pub use abm::team::Team;

pub mod problems;
