[![Build Status](https://travis-ci.com/THREDgroup/CISAT-rs.svg?branch=master)](https://travis-ci.com/THREDgroup/CISAT-rs)
[![Crates.io](https://img.shields.io/crates/v/cisat.svg)](https://crates.io/crates/cisat)
[![docs.rs](https://docs.rs/cisat/badge.svg)](https://docs.rs/cisat)
# About
This is an implementation of the Cognitively-Inspired Simulated Annealing Teams (CISAT) Framework in Rust. 

This is currently an incomplete implementation. Progress on CISAT characteristics includes:
- [x] Multi-agency
- [x] Organic interaction timing
- [x] Quality-informed solutions haring
- [x] Quality bias reduction
- [x] Self-bias
- [ ] Operational learning
- [ ] Locally-sensitive search
- [ ] Satisficing

# Usage
Here is a basic examples of usage

```rust
use cisat::{Cohort, Parameters, problems::Ackley};
fn main() {
    let mut x = Cohort::<Ackley>::new(Parameters::default());

    x.solve();

    println!("{:?}", x);
}
```
You can also implement new problem, agent, and team types using the `Solution`, `AgentMethods`, and `TeamMethods` 
traits, respectively. This allows significant flexibility within the basic CISAT structure.

# Literature
Aspects of CISAT have been published in several places. You can learn more about it here:
1. McComb, C., Cagan, J., & Kotovsky, K. (2015). Lifting the Veil: Drawing insights about design teams from a cognitively-inspired computational model. Design Studies, 40, 119-142. doi:[10.1016/j.destud.2015.06.005](https://doi.org/10.1016/j.destud.2015.06.005). [PDF](https://github.com/THREDgroup/CISAT-rs/blob/master/literature/2015_DesignStudies_LiftingTheVeil.pdf)
1. McComb, C., Cagan, J., & Kotovsky, K. (2016). Drawing inspiration from human design teams for better search and optimization: The heterogeneous simulated annealing teams algorithm. Journal of Mechanical Design, 138(4). doi:[10.1115/1.4032810](https://doi.org/10.1115/1.4032810). [PDF](https://github.com/THREDgroup/CISAT-rs/blob/master/literature/2016_JMD_HSAT.pdf)
2. McComb, C., Cagan, J., & Kotovsky, K. (2017). Capturing human sequence-learning abilities in configuration design tasks through markov chains. Journal of Mechanical Design, 139(9). doi:[10.1115/1.4037185](https://doi.org/10.1115/1.4037185). [PDF](https://github.com/THREDgroup/CISAT-rs/blob/master/literature/2017_JMD_MarkovChain.pdf)
1. McComb, C., Cagan, J., & Kotovsky, K. (2017). Optimizing design teams based on problem properties: computational team simulations and an applied empirical test. Journal of Mechanical Design, 139(4). doi:[10.1115/1.4035793](https://doi.org/10.1115/1.4035793). [PDF](https://github.com/THREDgroup/CISAT-rs/blob/master/literature/2017_JMD_OptimizingTeams.pdf)