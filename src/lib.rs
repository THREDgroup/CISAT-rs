#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

mod utilities;
pub use utilities::{
    parameters::{OperationalLearning, Parameters, TemperatureSchedule},
    Solution,
};

mod abm;
pub use abm::cohort::{build_cohort, Cohort};
pub use abm::team::{build_team, Team};

pub mod problems;
pub use problems::ackley::Ackley; //, example::Example, structure::Structure};
