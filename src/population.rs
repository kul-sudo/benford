use crate::histogram::save_histogram;
use std::{fs::read_to_string, io::stdin};

pub fn population() {
    let contents = read_to_string("data/population").unwrap();

    let mut statistics: [u128; 10] = [0; 10];

    for line in contents.lines() {
        statistics[line.chars().next().unwrap().to_digit(10).unwrap() as usize] += 1;
    }

    let sum = statistics.iter().sum::<u128>();
    save_histogram(
        10,
        statistics.map(|x| x as f64 / sum as f64).to_vec(),
        "population.png",
    )
    .unwrap();
}
