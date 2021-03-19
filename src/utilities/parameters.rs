//! This module contains the Parameters struct and a number of enums
use std::fmt;
// use strum_macros::EnumString;

/// This enum carries temperature schedule options
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum TemperatureSchedule {
    /// This is the Triki temperature schedule
    Triki {
        /// Initial temperature
        initial_temperature: f64,
        /// Delta updating parameter
        delta: f64,
        /// How long to dwell during temperature
        dwell: usize,
    },
    /// This is the Cauchy temperature schedule
    Cauchy {
        /// Initial temperature
        initial_temperature: f64,
        /// Delta updating parameter
        delta: f64,
        /// How long to dwell during temperature
        dwell: usize,
    },
    /// This is the geometry temperature schedule
    Geometric {
        /// Initial temperature
        initial_temperature: f64,
        /// How long to dwell during temperature
        dwell: usize,
    },
    /// I guess you could just not use a temperature schedule
    None,
}

/// This enum contains options for how the agent learns
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum OperationalLearning {
    /// Reinforcement over the action set as a multinomial distribution
    Multinomial {
        /// Learning rate
        learning_rate: f64,
        /// Initial move weights
        initial_learning_matrix: Vec<f64>,
    },
    /// Reinforcement over a sequence-based action set
    Markov {
        /// Learning rate
        learning_rate: f64,
        /// Initial transition matrix
        initial_learning_matrix: Vec<Vec<f64>>,
    },
    /// Reinforcement over an HMM-based action set
    HiddenMarkov {
        /// Learning rate
        learning_rate: f64,
        /// Initial transition matrix
        initial_transition_matrix: Vec<Vec<f64>>,
        /// Initial emission matrix
        initial_emission_matrix: Vec<Vec<f64>>,
    },
    /// Do you want dumb agents? This is how you get dumb agents.
    None,
}

/// This enum contains options for agent interaction
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum CommunicationStyle {
    /// Interaction based on a constatn frequency value
    ConstantFrequency {
        /// Frequency value
        frequency: f64,
    },
    /// Interaction at regular intervals
    RegularInterval {
        /// Interval value
        interval: usize,
    },
    /// Interaction at scheduled times
    ScheduledMeetings {
        /// Scheduled times
        times: Vec<usize>,
    },
    /// Do you want isolated agents? This is how you get isolated agents
    None,
}

/// This parameters struct. This tells CISAT what to do
#[derive(Clone, Debug)]
pub struct Parameters {
    /// Number of teams
    pub number_of_teams: usize,
    /// Number of agents on each team
    pub number_of_agents: usize,
    /// The number of iterations the team will run for
    pub number_of_iterations: usize,
    /// The temperature schedule to use
    pub temperature_schedule: TemperatureSchedule,
    /// The operational learning style to use
    pub operational_learning: OperationalLearning,
    /// The communication style to use
    pub communication: CommunicationStyle,
    /// The self bias value to use
    pub self_bias: f64,
    /// The quality bias value to use
    pub quality_bias: f64,
    /// The satisficing fraction to use
    pub satisficing_fraction: f64,
}

impl Parameters {
    /// This function makes it possible to load settings from a file
    pub fn load_from_file(&mut self, file_name: String) {
        unimplemented!()
    }
    /// This checks to make sure that all values in Parameters are valid
    pub fn verify(&self) {
        if self.satisficing_fraction < 0.0 || self.satisficing_fraction > 1.0 {
            panic!("The satisficing fraction must be between 0 and 1 inclusive.");
        }
    }
    /// Returns values necessary to run HSAT
    pub fn hsat() -> Self {
        Parameters {
            temperature_schedule: TemperatureSchedule::Triki {
                initial_temperature: 100.0,
                delta: 0.05,
                dwell: 50,
            },
            communication: CommunicationStyle::RegularInterval { interval: 1 },
            self_bias: 0.0,
            quality_bias: 0.0,
            satisficing_fraction: 0.0,
            ..Default::default()
        }
    }
    /// Set teams value
    pub fn with_teams(mut self, number_of_teams: usize) -> Self {
        self.number_of_teams = number_of_teams;
        self
    }
    /// Set teams value
    pub fn with_agents(mut self, number_of_agents: usize) -> Self {
        self.number_of_agents = number_of_agents;
        self
    }
    /// Set teams value
    pub fn with_iters(mut self, number_of_iterations: usize) -> Self {
        self.number_of_iterations = number_of_iterations;
        self
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            number_of_teams: 1,
            number_of_agents: 3,
            number_of_iterations: 100,
            temperature_schedule: TemperatureSchedule::Geometric {
                initial_temperature: 1.0,
                dwell: 1,
            },
            operational_learning: OperationalLearning::None,
            communication: CommunicationStyle::None,
            self_bias: 1.0,
            quality_bias: 1.0,
            satisficing_fraction: 0.5,
        }
    }
}

#[allow(unused_must_use)]
impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " - {} teams", self.number_of_teams);
        writeln!(f, " - {} agents", self.number_of_agents);
        writeln!(f, " - {} iterations", self.number_of_iterations);

        match self.temperature_schedule {
            TemperatureSchedule::Triki {
                initial_temperature,
                delta,
                dwell,
            } => {
                writeln!(f, " - Triki annealing schedule");
                writeln!(f, "    - initial_temperature = {}", initial_temperature);
                writeln!(f, "    - delta = {}", delta);
                writeln!(f, "    - dwell = {}", dwell);
            }
            TemperatureSchedule::Cauchy {
                initial_temperature,
                delta,
                dwell,
            } => {
                writeln!(f, " - Cauchy annealing schedule");
                writeln!(f, "    - initial_temperature = {}", initial_temperature);
                writeln!(f, "    - delta = {}", delta);
                writeln!(f, "    - dwell = {}", dwell);
            }
            TemperatureSchedule::Geometric {
                initial_temperature,
                dwell,
            } => {
                writeln!(f, " - Geometric annealing schedule");
                writeln!(f, "    - initial_temperature = {}", initial_temperature);
                writeln!(f, "    - dwell = {}", dwell);
            }
            TemperatureSchedule::None => {
                writeln!(f, " - No annealing schedule");
            }
        }
        match &self.operational_learning {
            OperationalLearning::Multinomial {
                learning_rate,
                initial_learning_matrix,
            } => {
                writeln!(f, " - Multinomial learning style");
                writeln!(f, "    - learning rate = {}", learning_rate);
            }
            OperationalLearning::Markov {
                learning_rate,
                initial_learning_matrix,
            } => {
                writeln!(f, " - Markov Chain learning style");
                writeln!(f, "    - learning rate = {}", learning_rate);
            }
            OperationalLearning::HiddenMarkov { learning_rate, .. } => {
                writeln!(f, " - Hidden Markov learning style");
                writeln!(f, "    - learning rate = {}", learning_rate);
            }
            OperationalLearning::None => {
                writeln!(f, " - No operational learning");
            }
        }
        writeln!(f, " - self bias = {}", self.self_bias);
        writeln!(f, " - quality bias = {}", self.quality_bias);
        writeln!(f, " - satisficing fraction = {}", self.satisficing_fraction);
        Ok(())
    }
}
