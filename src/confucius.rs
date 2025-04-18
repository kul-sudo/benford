use crate::histogram::save_histogram;
use std::{collections::HashMap, fs::read_to_string};

pub fn confucius() {
    let contents = read_to_string("data/confucius").unwrap();

    let mut occurrences = HashMap::new();
    for char in contents.chars() {
        occurrences.entry(char).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut statistics: [u128; 10] = [0; 10];
    for frequency in occurrences.values() {
        statistics[frequency
            .to_string()
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize] += 1;
    }

    let sum = statistics[1..].iter().sum::<u128>();
    save_histogram(
        10,
        statistics.map(|x| x as f64 / sum as f64).to_vec(),
        "confucius.png",
    )
    .unwrap();
}
