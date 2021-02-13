use cisat::problems::Ackley;
use cisat::{Cohort, Parameters};
fn main() {
    let mut x = Cohort::<Ackley>::new(Parameters::default());

    x.solve();

    println!("{:?}", x);
}
