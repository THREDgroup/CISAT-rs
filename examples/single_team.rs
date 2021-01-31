use cisat;

fn main() {
    let mut x: cisat::Team<cisat::problems::Ackley> = cisat::build_team(cisat::Parameters {
        number_of_teams: 1,
        number_of_agents: 5,
        number_of_iterations: 1000,
        temperature_schedule: cisat::TemperatureSchedule::Cauchy {
            initial_temperature: 10.0,
        },
        operational_learning: cisat::OperationalLearning::HiddenMarkov {
            learning_rate: 0.05,
        },
        self_bias: 0.0,
        quality_bias: 0.0,
        satisficing_fraction: 0.0,
    });

    x.solve();

    println!("{}", x);
}
