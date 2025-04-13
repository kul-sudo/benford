use crate::histogram::save_histogram;
use std::fs::read_to_string;

const SEPARATOR_STRING: &str = "DMEUISFEDGNGEnyNGEGF*^ng679E67g9E^9nf7egwsn67f^G&EGN^EFN^G&FEG^&feGN^&g(F^6gfe7N^(GFBn9gN(^VE69gN^(GEng697efVG^97VGn6976n79VEFg6n79EYGFEHJEYUGgFEFEFEF][e]f[E}[EUHEG&*G]]))))"; // A string that can't occur in the given text

pub fn book() {
    let mut contents = read_to_string("data/tom_sawyer").unwrap();

    for escape_sequence in ['\n', '\t'] {
        contents = contents.replace(escape_sequence, " ");
    }

    while contents.contains("  ") {
        contents = contents.replace("  ", " ");
    }

    contents = contents.replace("“", r#"""#).replace("”", r#"""#);

    for substr in [". ", r#"." "#, r#"?" "#, r#"!" "#] {
        contents = contents.replace(substr, &(substr.to_string() + SEPARATOR_STRING));
    }

    let mut statistics: [u128; 10] = [0; 10];

    for sentence in contents.split(SEPARATOR_STRING).collect::<Vec<_>>() {
        statistics[(sentence.len() - 1)
            .to_string()
            .chars()
            .next()
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap()] += 1;
    }

    let n = statistics.iter().sum::<u128>();

    save_histogram(
        10,
        statistics.map(|x| x as f64 / n as f64).to_vec(),
        "book.png",
    )
    .unwrap();
}
