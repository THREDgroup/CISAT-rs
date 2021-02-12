//! This module contains the Cohort class, a container for multiple Teams

use super::{
    super::utilities::{parameters::Parameters, Solution},
    team::Team,
};
use rayon::prelude::*;

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

    /// This runs the cohort using parallelism
    pub fn solve(&mut self) {
        self.team_list.par_iter_mut().for_each(|x| x.solve());
    }

    /// This runs a single iteration
    pub fn iterate(&mut self) {
        self.team_list.iter_mut().for_each(|x| x.iterate());
    }

    /// Get the current best solution
    pub fn get_best_solution(&mut self) -> f64 {
        let mut best_solution = self.team_list[0].pull_best_solution();
        for i in 1..self.team_list.len() {
            if best_solution > self.team_list[i].pull_best_solution() {
                best_solution = self.team_list[i].pull_best_solution();
            }
        }
        best_solution.get_quality_scalar()
    }
}
