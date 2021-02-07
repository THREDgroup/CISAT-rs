//! This module contains the Team class, a set of interacting Agents

use super::{
    super::utilities::{parameters::Parameters, Solution},
    agent::Agent,
};
use crate::{OperationalLearning, TemperatureSchedule};

/// This is the Team construct, which contains a set of Agents
#[derive(Clone, Debug)]
pub(crate) struct Team<T> {
    /// The parameters that the team runs with
    parameters: Parameters,
    /// The agents contained in the team
    agent_list: Vec<Agent<T>>,
}

impl<T: Clone + Solution<T>> Team<T> {
    /// This generates a new team
    pub(crate) fn new(parameters: Parameters) -> Team<T> {
        let mut agents = vec![Agent::new(parameters.clone()); parameters.number_of_agents];
        for i in 1..parameters.number_of_agents {
            agents[i] = Agent::new(parameters.clone());
        }
        Team {
            parameters,
            agent_list: agents,
        }
    }

    /// This runs a bunch of iterations to solve
    pub(crate) fn solve(&mut self) {
        for _ in 0..self.parameters.number_of_iterations {
            self.iterate();
        }
    }

    /// This runs a single iteration
    pub(crate) fn iterate(&mut self) {
        for i in 0..self.parameters.number_of_agents {
            self.agent_list[i].iterate();
        }
    }

    /// This pulls out the best solution from teh team
    pub(crate) fn pull_best_solution(&mut self) -> T {
        let mut best_solution = self.agent_list[0].best_solution_so_far.clone();
        for i in 1..self.agent_list.len() {
            if best_solution > self.agent_list[i].best_solution_so_far {
                best_solution = self.agent_list[i].best_solution_so_far.clone();
            }
        }
        best_solution
    }
}
