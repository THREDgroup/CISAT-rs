#[non_exhaustive]

pub enum TemperatureSchedule {
    Triki{
        initial_temperature: f64,
        delta: f64
    },
    Cauchy{initial_temperature: f64},
    Geometric{initial_temperature: f64},
}

enum OperationalLearning {
    Multinomial{
        learning_rate: f64,
        initial_learning_matrix: Vec<f64>,
    },
    Markov{
        learning_rate: f64,
        initial_learning_matrix: Vec<Vec<f64>>,
    },
    HiddenMarkov,
}

pub struct Parameters {
    number_of_repetitions: u64,
    number_of_agents: u64,
    number_of_iterations: u64,
    temperature_schedule: TemperatureSchedule,
    operational_learning: OperationalLearning,
    self_bias: f64,
    quality_bias: f64,
    satisficing_fraction: f64,
}

impl Parameters {
    pub fn set_from_file(&mut self, file_name: String) {

    }
}

struct Agent<T> {
    /// Agent_id
    agent_id: u64,
    /// The iteration number as tracked by the agent
    iteration_number: u64,
    /// The lst operation performed by the agent
    last_operation: u64,
    /// The temperature used by the agent
    temperature: f64,
    current_solution_quality: f64,
    best_quality_so_far: f64,
    current_solution: T,
    markov_chain: Vec<Vec<f64>>,
    parameters: Parameters,
}

impl<T> Agent<T> {

    fn reset(&mut self) {

    }

    fn generate_candidate_solution(&mut self){
    }

    fn iterate(&mut self) {

    }

    fn update_temperature(&mut self) {
        match self.parameters.temperature_schedule {
            TemperatureSchedule::Triki {initial_temperature, delta } => {
                self.temperature = initial_temperature/(self.iteration_number as f64);
            }
            TemperatureSchedule::Cauchy {initial_temperature} => {
                self.temperature = initial_temperature/(self.iteration_number as f64);
            }
            TemperatureSchedule::Geometric {initial_temperature} => {
                self.temperature = initial_temperature/(self.iteration_number as f64);
            }
        }

    }
}

struct Team {

}

impl Team {
    fn solve(&mut self) {

    }

    fn reset(&mut self) {

    }

    fn iterate(&mut self) {

    }

    fn pull_best_solution(&mut self) {

    }
}