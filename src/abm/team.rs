//! This module contains the Team class, a set of interacting Agents

use super::{
    super::utilities::{parameters::Parameters, Solution},
    agent::{Agent, AgentMethods},
};

/// This is the Team construct, which contains a set of Agents
#[derive(Clone, Debug)]
pub(crate) struct Team<T> {
    /// The parameters that the team runs with
    parameters: Parameters,
    /// The agents contained in the team
    agent_list: Vec<Agent<T>>,
}

/// This is a trait for implementing new teams
pub trait TeamMethods<T: Solution<T>> {
    /// Generates a new agent
    fn new(parameters: Parameters) -> Self;
    /// Iterates on the solution
    fn iterate(&mut self);
    /// Solves all the way for a solution
    fn solve(&mut self);
    /// Gets the best solution found by the team so far
    fn get_best_solution_so_far(&mut self) -> T;
}

impl<T: Clone + Solution<T>> TeamMethods<T> for Team<T> {
    /// This generates a new team
    fn new(parameters: Parameters) -> Team<T> {
        Team {
            agent_list: (0..parameters.number_of_agents)
                .map(|_| Agent::new(parameters.clone()))
                .collect(),
            parameters,
        }
    }

    /// This runs a single iteration
    fn iterate(&mut self) {
        self.agent_list.iter_mut().for_each(|x| x.iterate());
    }

    /// This runs a bunch of iterations to solve
    fn solve(&mut self) {
        for _ in 0..self.parameters.number_of_iterations {
            self.iterate();
        }
    }

    /// This pulls out the best solution from the team
    fn get_best_solution_so_far(&mut self) -> T {
        (0..self.parameters.number_of_agents)
            .map(|i| self.agent_list[i].get_best_solution_so_far())
            .collect::<Vec<T>>()
            .into_iter()
            .max()
            .unwrap()
    }
}
