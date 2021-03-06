//! This module contains the Agent class, the fundamental building block of CISAT
//! ```
//! ```

use super::super::utilities::{
    parameters::{OperationalLearning, Parameters, TemperatureSchedule},
    Solution,
};
use crate::utilities::randomness::{multinomial_draw, random_unit_draw};

/// This is an agent, the fundamental building block for a CISAT team
#[derive(Clone, Debug)]
pub struct Agent<S: Solution> {
    /// Agent id
    id: usize,
    /// The iteration number as tracked by the agent
    iteration_number: u64,
    /// The lst operation performed by the agent
    last_operation: u64,
    /// The temperature used by the agent
    temperature: f64,
    /// The current solution quality scalar of the agent
    current_solution_quality: f64,
    /// The current solution of the agent
    current_solution: S,
    /// The best quality so far for the agent
    best_quality_so_far: f64,
    /// The best solution so far for the agent
    best_solution_so_far: S,
    /// The parameters container
    parameters: Parameters,
}

/// This is a trait for implementing new agents
pub trait AgentMethods<S: Solution>: Send {
    /// Generates a new agent
    fn new(id: usize, parameters: Parameters) -> Self;
    /// Iterates on the solution
    fn iterate(&mut self);
    /// Gets the best solution found by the agent so far
    fn get_best_solution_so_far(&mut self) -> S;
    /// Gets the current solution of the agent
    fn get_current_solution(&mut self) -> S;
    /// Agent accepts rival solutions and interacts
    fn communicate(&mut self, solutions: Vec<S>);
}

impl<S: Solution> AgentMethods<S> for Agent<S> {
    fn new(id: usize, parameters: Parameters) -> Self {
        let solution = S::new();
        Agent {
            id,
            iteration_number: 1,
            last_operation: 0,
            temperature: 0.0,
            current_solution_quality: solution.get_quality_scalar(),
            best_quality_so_far: solution.get_quality_scalar(),
            best_solution_so_far: solution.clone(),
            current_solution: solution.clone(),
            parameters,
        }
    }

    fn iterate(&mut self) {
        // Update temperature
        self.update_temperature();

        // Generate a candidate
        let candidate = self.generate_candidate_solution();

        // Compare candidate
        if candidate > self.current_solution {
            self.current_solution = candidate;
        } else {
            let delta = candidate.clone() - self.current_solution.clone();
            let acceptance_probability = (delta / self.temperature).exp();
            if random_unit_draw() < acceptance_probability {
                self.current_solution = candidate;
            }
        }

        self.update_learning();

        // Update best solution
        if self.current_solution > self.best_solution_so_far {
            self.best_solution_so_far = self.current_solution.clone();
            self.best_quality_so_far = self.best_solution_so_far.get_quality_scalar();
        }

        // Increment iteration number
        self.iteration_number += 1;
    }

    fn get_best_solution_so_far(&mut self) -> S {
        self.best_solution_so_far.clone()
    }

    fn get_current_solution(&mut self) -> S {
        self.current_solution.clone()
    }

    fn communicate(&mut self, mut solutions: Vec<S>) {
        // Get scalar vector
        let mut qualities: Vec<f64> = solutions
            .clone()
            .into_iter()
            .map(|x| x.get_quality_scalar() + self.parameters.quality_bias)
            .collect();

        // Add in self-bias
        qualities[self.id] += self.parameters.self_bias;

        // Choose solution
        let idx = multinomial_draw(qualities);

        // Extract the design
        self.current_solution = solutions.remove(idx);
    }
}

impl<S: Solution> Agent<S> {
    /// This generates a new candidate solution for the agent
    fn generate_candidate_solution(&mut self) -> S {
        let mut candidate = self.current_solution.clone();
        candidate.apply_move_operator(0, 1.0);
        candidate
    }

    /// This updates the agent's learning
    fn update_learning(&mut self) {
        match self.parameters.operational_learning {
            OperationalLearning::Multinomial { .. } => {}
            OperationalLearning::Markov { .. } => {}
            OperationalLearning::HiddenMarkov { .. } => {}
            _ => {}
        }
    }

    /// This updates the agent's temperature
    fn update_temperature(&mut self) {
        match self.parameters.temperature_schedule {
            TemperatureSchedule::Triki {
                initial_temperature,
                delta,
                dwell,
            } => {
                self.temperature = initial_temperature / (self.iteration_number as f64);
            }
            TemperatureSchedule::Cauchy {
                initial_temperature,
                delta,
                dwell,
            } => {
                self.temperature = initial_temperature / (self.iteration_number as f64);
            }
            TemperatureSchedule::Geometric {
                initial_temperature,
                dwell,
            } => {
                self.temperature = initial_temperature / (self.iteration_number as f64);
            }
            _ => {}
        }
    }
}
