use cisat::{problems::Ackley, Cohort, CommunicationStyle, Parameters};

fn main() {
    let mut x = Cohort::<Ackley>::new(Parameters {
        number_of_teams: 1,
        communication: CommunicationStyle::RegularInterval { interval: 5 },
        ..Default::default()
    });

    x.solve();

    println!("{:?}", x);
}
