//! This module contains the Team class, a set of interacting Agents

use super::{
    super::utilities::{parameters::Parameters, randomness::random_unit_draw, Solution},
    agent::AgentMethods,
};
use crate::{Agent, CommunicationStyle};
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
    /// iteration number counter
    iteration_number: usize,
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
    /// Tell the team to talk
    fn communicate(&mut self);
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
                .map(|i| A::new(i, parameters.clone()))
                .collect(),
            parameters,
            iteration_number: 1,
            solution_type: Default::default(),
        }
    }

    /// This runs a single iteration
    fn iterate(&mut self) {
        // Check if its time to interact
        match &self.parameters.communication {
            CommunicationStyle::ConstantFrequency { frequency } => {
                if random_unit_draw() < *frequency {
                    self.communicate();
                }
            }
            CommunicationStyle::RegularInterval { interval } => {
                if self.iteration_number % interval == 0 {
                    self.communicate();
                }
            }
            CommunicationStyle::ScheduledMeetings { times } => {
                if times.into_iter().any(|i| *i == self.iteration_number) {
                    self.communicate();
                }
            }
            CommunicationStyle::None => {}
        }

        // Then iterate the agents
        self.agent_list.iter_mut().for_each(|x| x.iterate());

        // Increment iteration number
        self.iteration_number += 1;
    }

    fn communicate(&mut self) {
        // Get the solutions
        let solutions: Vec<S> = self
            .agent_list
            .iter_mut()
            .map(|x| x.get_current_solution())
            .collect();

        // Share the solutions
        self.agent_list
            .iter_mut()
            .for_each(|x| x.communicate(solutions.clone()));
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

impl<S, A> Default for Team<S, A>
where
    S: Solution,
    A: AgentMethods<S>,
{
    fn default() -> Self {
        Team::new(Default::default())
    }
}
