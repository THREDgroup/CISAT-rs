use cisat::{problems::Ackley, Cohort, CommunicationStyle, Parameters};

fn main() {
    type PROBLEM = Ackley<5>;
    let mut x = Cohort::<PROBLEM>::new(Parameters {
        number_of_teams: 1,
        communication: CommunicationStyle::RegularInterval { interval: 5 },
        ..Default::default()
    });

    x.solve();

    println!("{:?}", x);
}
