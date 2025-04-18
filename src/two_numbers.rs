use crate::histogram::save_histogram;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::ops::Range;

const N_RANGE: Range<u128> = 1..500000;
const ITERATIONS: usize = 5000000;

pub fn two_numbers() {
    let mut rng = StdRng::from_os_rng();
    let mut statistics: [u128; 10] = [0; 10];

    for _ in 0..ITERATIONS {
        let a = rng.random_range(N_RANGE);
        let b = rng.random_range(N_RANGE);

        statistics[(a * b)
            .to_string()
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize] += 1;
    }

    let sum = statistics.iter().sum::<u128>();
    save_histogram(
        10,
        statistics.map(|x| x as f64 / sum as f64).to_vec(),
        "two_numbers.png",
    )
    .unwrap();
}
