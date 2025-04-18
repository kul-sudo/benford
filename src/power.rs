use crate::histogram::save_histogram;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::ops::Range;

const N_RANGE: Range<u128> = 1..500;
const ITERATIONS: usize = 5000000;
const BASE: f64 = 1.1;

pub fn power() {
    let mut rng = StdRng::from_os_rng();
    let mut statistics: [u128; 10] = [0; 10];

    for _ in 0..ITERATIONS {
        let a = rng.random_range(N_RANGE) as f64;
        let b = BASE.powf(a);

        statistics[b.to_string().chars().next().unwrap().to_digit(10).unwrap() as usize] += 1;
    }

    let sum = statistics.iter().sum::<u128>();
    save_histogram(
        10,
        statistics.map(|x| x as f64 / sum as f64).to_vec(),
        "power.png",
    )
    .unwrap();
}
