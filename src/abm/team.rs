use super::{
    super::utilities::{parameters::Parameters, Solution},
    agent::Agent,
};
use crate::{OperationalLearning, TemperatureSchedule};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Team<T> {
    pub parameters: Parameters,
    agent_list: Vec<Agent<T>>,
}

impl<T: Clone + Solution<T>> Team<T> {
    pub fn new(parameters: Parameters) -> Team<T> {
        let mut agents = vec![Agent::new(parameters.clone()); parameters.number_of_agents];
        for i in 1..parameters.number_of_agents {
            agents[i] = Agent::new(parameters.clone());
        }
        Team {
            parameters,
            agent_list: agents,
        }
    }

    pub fn solve(&mut self) {
        for _ in 0..self.parameters.number_of_iterations {
            self.iterate();
        }
    }

    fn iterate(&mut self) {
        for i in 0..self.parameters.number_of_agents {
            self.agent_list[i].iterate();
        }
    }

    fn pull_best_solution(&mut self) -> T {
        let mut best_solution = self.agent_list[0].best_solution_so_far.clone();
        for i in 1..self.agent_list.len() {
            if best_solution > self.agent_list[i].best_solution_so_far {
                best_solution = self.agent_list[i].best_solution_so_far.clone();
            }
        }
        best_solution
    }
}

#[allow(unused_must_use)]
impl<T: Clone + Solution<T>> fmt::Display for Team<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Team:");
        writeln!(f, "\t{} agents", self.parameters.number_of_agents);
        writeln!(f, "\t{} iterations", self.parameters.number_of_iterations);

        match self.parameters.temperature_schedule {
            TemperatureSchedule::Triki {
                initial_temperature,
                delta,
            } => {
                writeln!(f, "\tTriki annealing schedule");
                writeln!(f, "\t\tinitial_temperature = {}", initial_temperature);
                writeln!(f, "\t\tdelta = {}", delta);
            }
            TemperatureSchedule::Cauchy {
                initial_temperature,
            } => {
                writeln!(f, "\tCauchy annealing schedule");
                writeln!(f, "\t\tinitial_temperature = {}", initial_temperature);
            }
            TemperatureSchedule::Geometric {
                initial_temperature,
            } => {
                writeln!(f, "\tGeometric annealing schedule");
                writeln!(f, "\t\tinitial_temperature = {}", initial_temperature);
            }
            TemperatureSchedule::None => {
                writeln!(f, "\tNo annealing schedule");
            }
        }
        match &self.parameters.operational_learning {
            OperationalLearning::Multinomial {
                learning_rate,
                initial_learning_matrix,
            } => {
                writeln!(f, "\tMultinomial learning style");
                writeln!(f, "\t\tlearning rate = {}", learning_rate);
            }
            OperationalLearning::Markov {
                learning_rate,
                initial_learning_matrix,
            } => {
                writeln!(f, "\tMultinomial learning style");
                writeln!(f, "\t\tlearning rate = {}", learning_rate);
            }
            OperationalLearning::HiddenMarkov { learning_rate, .. } => {
                writeln!(f, "\tMultinomial learning style");
                writeln!(f, "\t\tlearning rate = {}", learning_rate);
            }
            OperationalLearning::None => {
                writeln!(f, "\tNo operational learning");
            }
        }
        writeln!(f, "\tself bias = {}", self.parameters.self_bias);
        writeln!(f, "\tquality bias = {}", self.parameters.quality_bias);
        writeln!(
            f,
            "\tsatisficing fraction = {}",
            self.parameters.satisficing_fraction
        );

        writeln!(f, "Agents in team:");
        for i in 0..self.agent_list.len() {
            writeln!(f, "\tAgent {}:", i);
            writeln!(f, "\t\ttemperature = {}", self.agent_list[i].temperature);
            writeln!(
                f,
                "\t\tquality = {}",
                self.agent_list[i].best_quality_so_far
            );
        }

        Ok(())
    }
}
