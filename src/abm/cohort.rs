//! This module contains the Cohort class, a container for multiple Teams

use super::{
    super::utilities::{parameters::Parameters, Solution},
    team::{Team, TeamMethods},
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
        Cohort {
            team_list: (0..parameters.number_of_agents)
                .map(|_| Team::new(parameters.clone()))
                .collect(),
            parameters,
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
    pub fn get_best_solution_so_far(&mut self) -> f64 {
        (0..self.parameters.number_of_teams)
            .map(|i| self.team_list[i].get_best_solution_so_far())
            .collect::<Vec<T>>()
            .into_iter()
            .max()
            .unwrap()
            .get_quality_scalar()
    }
}
