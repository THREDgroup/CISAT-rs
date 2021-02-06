//! This module contains the Parameters struct and a number of enums

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
    },
    /// This is the Cauchy temperature schedule
    Cauchy {
        /// Initial temperature
        initial_temperature: f64,
    },
    /// This is the geometry temperature schedule
    Geometric {
        /// Initial temperature
        initial_temperature: f64,
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
    /// The oeprational learing style to use
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
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            number_of_teams: 1,
            number_of_agents: 3,
            number_of_iterations: 100,
            temperature_schedule: TemperatureSchedule::Geometric {
                initial_temperature: 1.0,
            },
            operational_learning: OperationalLearning::None,
            communication: CommunicationStyle::None,
            self_bias: 1.0,
            quality_bias: 1.0,
            satisficing_fraction: 0.5,
        }
    }
}
