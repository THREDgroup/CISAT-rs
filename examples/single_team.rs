use cisat;

fn main() {
    let mut x: cisat::Team<cisat::problems::Ackley> = cisat::Team::new(cisat::Parameters {
        number_of_teams: 1,
        number_of_agents: 10,
        number_of_iterations: 1000,
        temperature_schedule: cisat::TemperatureSchedule::Cauchy {
            initial_temperature: 10.0,
        },
        operational_learning: cisat::OperationalLearning::None,
        self_bias: 0.0,
        quality_bias: 0.0,
        satisficing_fraction: 0.0,
        communication: cisat::CommunicationStyle::None,
    });

    x.solve();

    println!("{}", x);
}
