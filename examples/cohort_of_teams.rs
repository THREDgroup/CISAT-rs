fn main() {
    let mut x = cisat::Cohort::<cisat::problems::Ackley>::new(cisat::Parameters::default());

    x.solve();

    println!("{:?}", x);
}
