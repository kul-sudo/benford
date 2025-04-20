use crate::histogram::save_histogram;
use json::parse;
use std::fs::{read_dir, read_to_string};

pub fn channels() {
    for name in ["suspicious_channel", "good_channel"] {
        let mut statistics: [u128; 10] = [0; 10];

        for entry in read_dir(format!("data/{}", name)).unwrap() {
            let data = read_to_string(entry.unwrap().path()).unwrap();
            let parsed = parse(&data).unwrap();

            if let Some(view_count) = parsed["view_count"].as_u64() {
                if let Some(comment_count) = parsed["comment_count"].as_u64() {
                    let ratio = comment_count as f64 / view_count as f64;
                    for char in ratio.to_string().chars() {
                        if char != '0' && char != '.' {
                            statistics[char.to_digit(10).unwrap() as usize] += 1;
                            break;
                        }
                    }
                }
            }
        }

        let sum = statistics[1..].iter().sum::<u128>();
        save_histogram(
            10,
            statistics.map(|x| x as f64 / sum as f64).to_vec(),
            &format!("{}.png", name),
        )
        .unwrap();
    }
}
