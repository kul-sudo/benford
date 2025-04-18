use crate::histogram::save_histogram;
use std::{io::stdin, time::Instant};
use termion::{event::Key, input::TermRead};

pub fn press_intervals() {
    println!(
        "Please make AT LEAST 200 presses! Imitate typing sentences from daily life. Repetitions aren't included. Use Enter to get the results."
    );

    let stdin = stdin();

    let mut last_press_timestamp = Instant::now();
    let mut statistics: [u128; 10] = [0; 10];

    let mut last_key = None;

    for c in stdin.keys() {
        let key = c.unwrap();
        if key == Key::Char('\n') {
            break;
        }

        if !last_key.is_some_and(|last_key| last_key == key) {
            statistics[last_press_timestamp
                .elapsed()
                .as_nanos()
                .to_string()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize] += 1;

            last_press_timestamp = Instant::now();
        }

        last_key = Some(key);
    }

    let n = statistics.iter().sum::<u128>();
    save_histogram(
        10,
        statistics.map(|x| x as f64 / n as f64).to_vec(),
        "press_intervals.png",
    )
    .unwrap();

    println!("{} key(s) pressed. Histogram saved.", n);
}
