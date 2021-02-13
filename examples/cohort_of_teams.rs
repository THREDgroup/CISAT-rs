fn main() {
    type S = cisat::problems::Ackley;
    type A = cisat::Agent<S>;
    type T = cisat::Team<S, A>;
    let mut x = cisat::Cohort::<S, A, T>::new(cisat::Parameters::default());

    x.solve();

    println!("{:?}", x);
}
