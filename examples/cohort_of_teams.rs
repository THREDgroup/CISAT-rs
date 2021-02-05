fn main() {
    let mut x = cisat::Cohort::<cisat::problems::Ackley>::new(cisat::Parameters {
        number_of_teams: 10,
        number_of_agents: 5,
        number_of_iterations: 10,
        temperature_schedule: cisat::TemperatureSchedule::Cauchy {
            initial_temperature: 100.0,
        },
        operational_learning: cisat::OperationalLearning::None,
        communication: cisat::CommunicationStyle::None,
        self_bias: 0.0,
        quality_bias: 0.0,
        satisficing_fraction: 0.0,
    });

    x.solve();

    println!("{:?}", x);
}
