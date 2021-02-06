//! This module contains the Cohort class, a container for multiple Teams

use super::{
    super::utilities::{parameters::Parameters, Solution},
    team::Team,
};

/// This is the Cohort class, a container for multiple teams
#[derive(Clone, Debug)]
pub struct Cohort<T> {
    /// This contains the parameters for the class
    parameters: Parameters,
    /// This contains the teams in the cohort
    team_list: Vec<Team<T>>,
}

impl<T: Clone + Solution<T>> Cohort<T> {
    /// This generates a new cohort
    pub fn new(parameters: Parameters) -> Cohort<T> {
        let teams = vec![Team::new(parameters.clone()); parameters.number_of_teams];
        Cohort {
            parameters,
            team_list: teams,
        }
    }

    /// This runs the cohort
    pub fn solve(&mut self) {
        for i in 0..self.parameters.number_of_teams {
            self.team_list[i].solve();
        }
    }
}
