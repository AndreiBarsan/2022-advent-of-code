use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

const MESSAGE_MARKER_SIZE: usize = 14;
const PACKET_MARKER_SIZE: usize = 4;

/// Specialized version of `all_diff` for the packer markers, which are only four characters.
fn all_diff_tup(tup: &(char, char, char, char)) -> bool {
    tup.0 != tup.1
        && tup.0 != tup.2
        && tup.0 != tup.3
        && tup.1 != tup.2
        && tup.1 != tup.3
        && tup.2 != tup.3
}

fn all_diff(elements: &[char]) -> bool {
    HashSet::<&char>::from_iter(elements.iter()).len() != elements.len()
}

fn day_06_tuning_trouble() -> (usize, usize) {
    let in_fpath = "input/06.txt";
    let sample = fs::read_to_string(in_fpath).expect("Could not read input file!");
    let input_chars: Vec<char> = sample.chars().collect();

    let part1_answer = input_chars
        .iter()
        .copied()
        .tuple_windows()
        .take_while(|tup| !all_diff_tup(tup))
        .count()
        + PACKET_MARKER_SIZE;

    let part2_answer = input_chars
        .windows(MESSAGE_MARKER_SIZE)
        .take_while(|&x| all_diff(x))
        .count()
        + MESSAGE_MARKER_SIZE;

    (part1_answer, part2_answer)
}

fn main() {
    let (part1_answer, part2_answer) = day_06_tuning_trouble();
    println!("{}", part1_answer);
    println!("{}", part2_answer);
}
