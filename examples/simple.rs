use cisat;

fn main() {
    let mut x: cisat::Team<cisat::problems::Ackley> = cisat::build_team(cisat::Parameters {
        number_of_repetitions: 0,
        number_of_agents: 0,
        number_of_iterations: 10,
        temperature_schedule: cisat::TemperatureSchedule::Cauchy {
            initial_temperature: 100.0,
        },
        operational_learning: cisat::OperationalLearning::HiddenMarkov,
        self_bias: 0.0,
        quality_bias: 0.0,
        satisficing_fraction: 0.0,
    });

    x.solve();

    println!("{:?}", x);
}
