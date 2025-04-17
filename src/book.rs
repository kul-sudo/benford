use crate::histogram::save_histogram_split;
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

    for substr in [". ", r#".” "#, r#"?” "#, r#"!” "#] {
        contents = contents.replace(substr, &(substr.to_string() + SEPARATOR_STRING));
    }

    let mut lengths = Vec::new();

    for sentence in contents.split(SEPARATOR_STRING) {
        if !sentence.is_empty() {
            lengths.push(sentence.len() as u128);
        }
    }

    save_histogram_split(lengths, "book.png").unwrap();
}
