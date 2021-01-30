use super::super::utilities::{parameters::Parameters, Solution};
use super::agent::{build_agent, Agent};

#[derive(Clone, Debug)]
pub struct Team<T> {
    pub parameters: Parameters,
    agent_list: Vec<Agent<T>>,
}

impl<T: Clone + Solution<T>> Team<T> {
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

    fn pull_best_solution(&mut self) {}
}

pub fn build_team<T: Clone + Solution<T>>(parameters: Parameters) -> Team<T> {
    let agents = vec![build_agent(0, parameters.clone()); 1];
    Team {
        parameters,
        agent_list: agents,
    }
}
