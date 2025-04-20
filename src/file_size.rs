use crate::histogram::save_histogram;
use std::{fs::read_dir, path::PathBuf};

pub fn walk_dir(statistics: &mut [u128; 10], starting_dir: &PathBuf) {
    for component in read_dir(starting_dir).unwrap() {
        let entry = component.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            walk_dir(statistics, &entry.path());
        } else if file_type.is_file() || file_type.is_symlink() {
            statistics[entry
                .metadata()
                .unwrap()
                .len()
                .to_string()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize] += 1;
        }
    }
}

pub fn file_size() {
    let mut statistics: [u128; 10] = [0; 10];
    walk_dir(&mut statistics, &PathBuf::from("/home/user/tmp"));

    let files_n = statistics[1..].iter().sum::<u128>();
    println!("Files N: {}", files_n);
    save_histogram(
        10,
        statistics.map(|x| x as f64 / files_n as f64).to_vec(),
        "file_size.png",
    )
    .unwrap();
}
