use cisat::{CommunicationStyle, OperationalLearning, Parameters, TemperatureSchedule};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::time::Instant;
use structopt::StructOpt;

/// Simulates team problem-solving using the Cognitively-Inspired Simulated Annealing Teams (CISAT) framework.
#[derive(StructOpt, Debug)]
#[structopt(author, name = "CISAT")]
struct Cli {
    /// Makes CISAT very, very chatty
    #[structopt(short, long)]
    verbosity: bool,
    /// The number of teams to run
    #[structopt(short = "T", long, default_value = "10")]
    teams: usize,
    /// The problem to simulate solving (Ackley or Structure)
    #[structopt(short = "P", long)]
    problem: String,
    /// The number of agents on each team
    #[structopt(short = "A", long, default_value = "3")]
    agents: usize,
    /// The number of iterations
    #[structopt(short = "I", long, default_value = "100")]
    iter: usize,
    /// The temperature schedule to use
    #[structopt(short = "S", long, default_value = "Geometric")]
    schedule: String,
    /// The initial temperature
    #[structopt(short = "t", long, default_value = "10")]
    initial_temperature: f64,
    /// Triki temperature coefficient
    #[structopt(
        short = "d",
        long,
        default_value = "0.1",
        required_if("schedule", "Triki")
    )]
    pub delta: f64,
    /// The operational learing style to use (Multinomial, Markov, HiddenMarkov, or None)
    #[structopt(short = "L", long, default_value = "Markov")]
    pub learning: String,
    /// The temperature schedule to use (Geometric, Cauchy, or Triki)
    #[structopt(short = "r", long, default_value = "0.05")]
    pub learning_rate: f64,
    /// The self bias value to use
    #[structopt(short = "b", long, default_value = "1.0")]
    pub self_bias: f64,
    /// The quality bias value to use
    #[structopt(short = "q", long, default_value = "1.0")]
    pub quality_bias: f64,
    /// The satisficing fraction to use
    #[structopt(short = "s", long, default_value = "0.5")]
    pub satisficing: f64,
}

fn main() {
    // Parse args
    let args = Cli::from_args();

    // Match for temperature schedule
    let learning_style = match args.learning.to_lowercase().as_str() {
        "multinomial" => OperationalLearning::Multinomial {
            learning_rate: args.learning_rate,
            initial_learning_matrix: vec![],
        },
        "markov" => OperationalLearning::Markov {
            learning_rate: args.learning_rate,
            initial_learning_matrix: vec![],
        },
        "hiddenmarkov" => OperationalLearning::HiddenMarkov {
            learning_rate: args.learning_rate,
            initial_transition_matrix: vec![],
            initial_emission_matrix: vec![],
        },
        "none" => OperationalLearning::None,
        &_ => panic!(
            "{} is not a valid option for --learning",
            args.learning.as_str()
        ),
    };

    // Match for learning style
    let temperature_schedule = match args.schedule.to_lowercase().as_str() {
        "geometric" => TemperatureSchedule::Geometric {
            initial_temperature: args.initial_temperature,
        },
        "cauchy" => TemperatureSchedule::Cauchy {
            initial_temperature: args.initial_temperature,
        },
        "triki" => TemperatureSchedule::Triki {
            initial_temperature: args.initial_temperature,
            delta: args.delta,
        },
        "none" => TemperatureSchedule::None,
        &_ => panic!(
            "{} is not a valid option for --schedule",
            args.schedule.as_str()
        ),
    };

    // Things
    println!(
        "Solving the {} problem with following parameters",
        args.problem
    );
    // Generate parameters struct
    let params = Parameters {
        number_of_teams: args.teams,
        number_of_agents: args.agents,
        number_of_iterations: args.iter,
        temperature_schedule,
        operational_learning: learning_style,
        communication: CommunicationStyle::None,
        self_bias: args.self_bias,
        quality_bias: args.quality_bias,
        satisficing_fraction: args.satisficing,
    };

    println!("{}", params);

    // match for problem and run
    let started = Instant::now();
    let bar = ProgressBar::new(args.iter as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} [{msg}] {wide_bar} [{percent}%, ~{eta} remaining]"), // .progress_chars("##-"),
    );
    bar.set_message("Starting...");
    match args.problem.to_lowercase().as_str() {
        "ackley" => {
            let mut cisat = cisat::Cohort::<cisat::problems::Ackley>::new(params);
            // for _ in 1..args.iter {
            //     cisat.iterate();
            //     bar.set_message(format!("Best: {:.2}", cisat.get_best_solution()).as_str());
            //     bar.inc(1);
            // }
            cisat.solve();
            bar.finish_and_clear();
            println!(
                "Done! The simulation took {}, and the best solution found was {:.2}.",
                HumanDuration(started.elapsed()),
                cisat.get_best_solution_so_far()
            );
        }
        "structure" => {
            let mut cisat = cisat::Cohort::<cisat::problems::Structure>::new(params);
            for _ in 1..args.iter {
                cisat.iterate();
                bar.set_message(format!("Best: {:.2}", cisat.get_best_solution_so_far()).as_str());
                bar.inc(1);
            }
            bar.finish_and_clear();
            println!(
                "Done! The simulation took {}, and the best solution found was {:.2}.",
                HumanDuration(started.elapsed()),
                cisat.get_best_solution_so_far()
            );
        }
        &_ => panic!(
            "{} is not a valid option for --problem",
            args.schedule.as_str()
        ),
    }
}
