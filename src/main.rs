use cisat::{CommunicationStyle, OperationalLearning, Parameters, TemperatureSchedule};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::time::Instant;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The number of teams to run
    #[structopt(short = "t", long, default_value = "10")]
    teams: usize,
    /// The number of teams to run
    #[structopt(short = "p", long, default_value = "Ackley")]
    problem: String,
    /// The number of agents on each team
    #[structopt(short = "a", long, default_value = "3")]
    agents: usize,
    /// The number of iterations
    #[structopt(short = "i", long, default_value = "100")]
    iter: usize,
    /// The temperature schedule to use
    #[structopt(short = "d", long, default_value = "Geometric")]
    pub schedule: String,
    /// The temperature schedule to use
    #[structopt(short = "g", long, default_value = "100")]
    pub initial_temperature: f64,
    /// Triki temperature coefficient
    #[structopt(long, default_value = "0.1", required_if("schedule", "Triki"))]
    pub delta: f64,
    /// The operational learing style to use
    #[structopt(short = "l", long, default_value = "Markov")]
    pub learning: String,
    /// The communication style to use
    #[structopt(short = "c", long, default_value = "communication")]
    pub communication: String,
    /// The self bias value to use
    #[structopt(short = "b", long, default_value = "1.0")]
    pub self_bias: f64,
    /// The quality bias value to use
    #[structopt(short = "q", long, default_value = "1.0")]
    pub quality_bias: f64,
    /// The satisficing fraction to use
    #[structopt(short = "f", long, default_value = "0.5")]
    pub satisficing: f64,
}

fn main() {
    // Parse args
    let args = Cli::from_args();

    // Match for temperature schedule
    let temperature_schedule = match args.schedule.as_str() {
        "Geometric" => TemperatureSchedule::Geometric {
            initial_temperature: args.initial_temperature,
        },
        "Cauchy" => TemperatureSchedule::Cauchy {
            initial_temperature: args.initial_temperature,
        },
        "Triki" => TemperatureSchedule::Triki {
            initial_temperature: args.initial_temperature,
            delta: args.delta,
        },
        &_ => TemperatureSchedule::None,
    };

    // Generate parameters struct
    let params = Parameters {
        number_of_teams: args.teams,
        number_of_agents: args.agents,
        number_of_iterations: args.iter,
        temperature_schedule,
        operational_learning: OperationalLearning::None,
        communication: CommunicationStyle::None,
        self_bias: args.self_bias,
        quality_bias: args.quality_bias,
        satisficing_fraction: args.satisficing,
    };

    // match for problem and run
    println!(
        "Solving the {} problem with {} teams of {} agents",
        args.problem, params.number_of_teams, params.number_of_agents
    );
    let started = Instant::now();
    let bar = ProgressBar::new(args.iter as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} [{msg}] {wide_bar} [{percent}%, ~{eta} remaining]"), // .progress_chars("##-"),
    );
    bar.set_message("Starting...");
    match args.problem.as_str() {
        "Ackley" => {
            let mut cisat = cisat::Cohort::<cisat::problems::Ackley>::new(params);
            for i in 1..args.iter {
                cisat.iterate();
                bar.set_message(format!("Best: {:.2}", cisat.get_best_solution()).as_str());
                bar.inc(1);
            }
            bar.finish();
        }
        "Structure" => {
            let mut cisat = cisat::Cohort::<cisat::problems::Structure>::new(params);
            cisat.solve();
        }
        &_ => {
            panic!("Something has gone terribly wrong.");
        }
    }
    println!(
        "Done! The simulation took {}.",
        HumanDuration(started.elapsed())
    );
}
