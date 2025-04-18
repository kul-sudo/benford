use crate::histogram::save_histogram;
use json::{JsonValue, parse};
use std::fs::read_to_string;

pub fn comments() {
    let mut statistics: [u128; 10] = [0; 10];

    let data = read_to_string("data/video.json").unwrap();
    let parsed = parse(&data).unwrap();
    let comments = &parsed["comments"];
    if let JsonValue::Array(array) = comments {
        for comment in array {
            statistics[comment["text"]
                .to_string()
                .len()
                .to_string()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize] += 1;
        }
    };

    let sum = statistics.iter().sum::<u128>();
    save_histogram(
        10,
        statistics.map(|x| x as f64 / sum as f64).to_vec(),
        "comments.png",
    )
    .unwrap();
}
