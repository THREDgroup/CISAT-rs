//! This module contains the Team class, a set of interacting Agents

use super::{
    super::utilities::{parameters::Parameters, Solution},
    agent::AgentMethods,
};
use crate::Agent;
use std::marker::PhantomData;

/// This is the Team construct, which contains a set of Agents
#[derive(Clone, Debug)]
pub struct Team<S, A>
where
    S: Solution,
    A: AgentMethods<S>,
{
    /// The parameters that the team runs with
    parameters: Parameters,
    /// The agents contained in the team
    agent_list: Vec<A>,
    /// Bookkeeping the solution type
    solution_type: PhantomData<S>,
}

/// This is a trait for implementing new teams
pub trait TeamMethods<S: Solution, A: AgentMethods<S> = Agent<S>>: Send {
    /// Generates a new agent
    fn new(parameters: Parameters) -> Self;
    /// Iterates on the solution
    fn iterate(&mut self);
    /// Solves all the way for a solution
    fn solve(&mut self);
    /// Gets the best solution found by the team so far
    fn get_best_solution_so_far(&mut self) -> S;
}

impl<S, A> TeamMethods<S, A> for Team<S, A>
where
    S: Solution,
    A: AgentMethods<S>,
{
    /// This generates a new team
    fn new(parameters: Parameters) -> Self {
        Team {
            agent_list: (0..parameters.number_of_agents)
                .map(|_| A::new(parameters.clone()))
                .collect(),
            parameters,
            solution_type: Default::default(),
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
    fn get_best_solution_so_far(&mut self) -> S {
        (0..self.parameters.number_of_agents)
            .map(|i| self.agent_list[i].get_best_solution_so_far())
            .collect::<Vec<S>>()
            .into_iter()
            .max()
            .unwrap()
    }
}
