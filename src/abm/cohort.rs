use super::{
    super::utilities::{parameters::Parameters, Solution},
    team::Team,
};

#[derive(Clone, Debug)]
pub struct Cohort<T> {
    pub parameters: Parameters,
    team_list: Vec<Team<T>>,
}

impl<T: Clone + Solution<T>> Cohort<T> {
    pub fn new(parameters: Parameters) -> Cohort<T> {
        let teams = vec![Team::new(parameters.clone()); parameters.number_of_teams];
        Cohort {
            parameters,
            team_list: teams,
        }
    }

    pub fn solve(&mut self) {
        for i in 0..self.parameters.number_of_teams {
            self.team_list[i].solve();
        }
    }
}
