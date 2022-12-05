use std::fs;
use std::ops::RangeInclusive;

type ElfRange = RangeInclusive<i64>;

fn parse_range(input: &str) -> ElfRange {
    let parts: Vec<i64> = input
        .split('-')
        .map(|num| num.parse().expect("Could not parse number"))
        .collect();
    if parts.len() != 2 {
        panic!("Unexpected parts in range");
    }
    parts[0]..=parts[1]
}

/// Returns true if the LEFT range contains the right one
fn contains(l: &ElfRange, r: &ElfRange) -> bool {
    l.contains(r.start()) && l.contains(r.end())
}

fn overlap(l: &ElfRange, r: &ElfRange) -> bool {
    l.contains(r.start()) || l.contains(r.end()) || r.contains(l.start()) || r.contains(l.end())
}

fn parse_ranges(input: &str) -> (ElfRange, ElfRange) {
    let parts: Vec<&str> = input.split(',').collect();
    (parse_range(parts[0]), parse_range(parts[1]))
}

fn day_04_camp_cleanup() -> (i64, i64) {
    let a = 0..2;
    let in_fpath = "input/04.txt";
    let ranges: Vec<(ElfRange, ElfRange)> = fs::read_to_string(in_fpath)
        .expect("Could not read input file!")
        .lines()
        .map(parse_ranges)
        .collect();

    let part1_fully_contain: Vec<(ElfRange, ElfRange)> = ranges
        .clone()
        .into_iter()
        .filter(|(l, r)| contains(l, r) || contains(r, l))
        .collect();

    let part_2_overlap_at_all = ranges.clone().into_iter().filter(|(l, r)| overlap(l, r));

    (
        part1_fully_contain.len() as i64,
        part_2_overlap_at_all.count() as i64,
    )
}

fn main() {
    let (part1_answer, part2_answer) = day_04_camp_cleanup();
    println!("{}", part1_answer);
    println!("{}", part2_answer);
}
