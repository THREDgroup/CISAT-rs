use super::{
    super::utilities::{parameters::Parameters, Solution},
    team::{build_team, Team},
};

#[derive(Clone, Debug)]
pub struct Cohort<T> {
    pub parameters: Parameters,
    team_list: Vec<Team<T>>,
}

impl<T: Clone + Solution<T>> Cohort<T> {
    pub fn solve(&mut self) {
        for i in 0..self.parameters.number_of_repetitions {
            self.team_list[i].solve();
        }
    }
}

pub fn build_cohort<T: Clone + Solution<T>>(parameters: Parameters) -> Cohort<T> {
    let teams = vec![build_team(parameters.clone()); parameters.number_of_repetitions];
    Cohort {
        parameters,
        team_list: teams,
    }
}
