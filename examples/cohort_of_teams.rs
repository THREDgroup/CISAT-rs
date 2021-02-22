use cisat::problems::Ackley;
use cisat::CommunicationStyle;
use cisat::{Cohort, Parameters};
fn main() {
    let y = Parameters {
        number_of_teams: 1,
        communication: CommunicationStyle::RegularInterval { interval: 5 },
        ..Default::default()
    };
    let mut x = Cohort::<Ackley>::new(y);

    x.solve();

    println!("{:?}", x);
}
