use cisat::problems::Ackley;
use cisat::{Cohort, Parameters};
fn main() {
    let mut x = Cohort::<Ackley>::new(Parameters::default().with_teams(10));

    x.solve();

    println!("{:?}", x);
}
