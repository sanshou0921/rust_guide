use rand::{prelude::Distribution, rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use statrs::distribution::MultivariateNormal;

struct State<const N_DIM: usize> {
    rng: StdRng,
    arr: [f64; N_DIM],
    proposal_distribution: MultivariateNormal,
}

fn log_likelihood<const N_DIM: usize>(arr: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..N_DIM {
        sum += -(arr[i] * arr[i] / 2.0) as f64;
    }
    sum
}

impl<const N_DIM: usize> State<N_DIM> {
    fn new(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        let arr = [0.0; N_DIM];
        let mean = vec![0.0; N_DIM];
        let mut cov = Vec::<f64>::new();
        for i in 0..N_DIM {
            for j in 0..N_DIM {
                if i == j {
                    cov.push(1.0);
                } else {
                    cov.push(0.0);
                }
            }
        }
        let proposal_distribution = MultivariateNormal::new(mean, cov).unwrap();
        Self { rng, arr, proposal_distribution }
    }

    fn take_step(&mut self) {
        let binding = self.proposal_distribution.sample(&mut self.rng);
        let proposal= binding.as_slice();
        let log_alpha = log_likelihood::<N_DIM>(proposal) - log_likelihood::<N_DIM>(&self.arr);

        if log_alpha > (self.rng.gen_range(0.0..1.0) as f64).ln() {
            let mut new_location: [f64; N_DIM] = [0.0; N_DIM];
            for i in 0..N_DIM {
                new_location[i] = proposal[i];
            }
            self.arr = new_location;
        }
    }
}

fn main() {
    const N_STEP: usize = 100;
    const N_DIM: usize = 2;
    let mut state = State::<N_DIM>::new(0xdeadbeef);
    let result: [[f64; N_DIM]; N_STEP] = [[0.0; N_DIM]; N_STEP];
    for _ in 0..N_STEP {
        state.take_step();
        println!("{:?}", state.arr);
    }
}
