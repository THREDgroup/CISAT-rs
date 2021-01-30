#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum TemperatureSchedule {
    Triki {
        initial_temperature: f64,
        delta: f64,
    },
    Cauchy {
        initial_temperature: f64,
    },
    Geometric {
        initial_temperature: f64,
    },
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum OperationalLearning {
    Multinomial {
        learning_rate: f64,
        initial_learning_matrix: Vec<f64>,
    },
    Markov {
        learning_rate: f64,
        initial_learning_matrix: Vec<Vec<f64>>,
    },
    HiddenMarkov {
        learning_rate: f64,
    },
}

#[derive(Clone, Debug)]
pub struct Parameters {
    pub number_of_teams: usize,
    pub number_of_agents: usize,
    pub number_of_iterations: usize,
    pub temperature_schedule: TemperatureSchedule,
    pub operational_learning: OperationalLearning,
    pub self_bias: f64,
    pub quality_bias: f64,
    pub satisficing_fraction: f64,
}

impl Parameters {
    pub fn load_from_file(&mut self, file_name: String) {}
}