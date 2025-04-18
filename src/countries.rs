use crate::histogram::save_histogram;
use std::fs::read_to_string;

const UNIT_IN_KM: f64 = 50.0;

pub fn countries() {
    let contents = read_to_string("data/countries").unwrap();

    let mut statistics: [u128; 10] = [0; 10];

    for line in contents.lines() {
        let num = line.parse::<u128>().unwrap() as f64 / UNIT_IN_KM.powi(2);

        for char in num.to_string().chars() {
            if char != '0' && char != '.' {
                statistics[char.to_digit(10).unwrap() as usize] += 1;
                break;
            }
        }
    }

    let sum = statistics.iter().sum::<u128>();
    save_histogram(
        10,
        statistics.map(|x| x as f64 / sum as f64).to_vec(),
        "countries.png",
    )
    .unwrap();
}
