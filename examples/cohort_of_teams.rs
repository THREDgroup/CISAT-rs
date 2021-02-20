use cisat::problems::Ackley;
use cisat::{Cohort, Parameters};
fn main() {
    let mut x = Cohort::<Ackley>::new(
        Parameters::default()
            .with_teams(10)
            .with_agents(20)
            .with_iterations(10000),
    );

    x.solve();

    println!("{:?}", x);
}
