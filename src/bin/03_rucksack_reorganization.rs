use itertools::Itertools;
use std::{collections::HashMap, collections::HashSet, fs};

type Rucksack = (HashMap<char, usize>, HashMap<char, usize>);

fn parse_rucksack(line: &str) -> Rucksack {
    let len = line.len();
    let lh = line[0..len / 2].chars().counts();
    let rh = line[len / 2..].chars().counts();
    (lh, rh)
}

fn common_element(sack: &Rucksack) -> char {
    let left_keys: HashSet<char> = sack.0.keys().copied().collect();
    let right_keys: HashSet<char> = sack.1.keys().copied().collect();
    let common_opt = left_keys.intersection(&right_keys).into_iter().next();

    match common_opt {
        Some(ch) => *ch,
        None => panic!("No common element found between halves!"),
    }
}

fn element_priority(item: char) -> i64 {
    if ('a'..='z').contains(&item) {
        (item as i64) + 1 - ('a' as i64)
    } else if ('A'..='Z').contains(&item) {
        (item as i64) + 27 - ('A' as i64)
    } else {
        panic!("Invalid rucksack element: {}", item);
    }
}

fn find_badge(rs: &[Rucksack]) -> char {
    let all: Vec<HashSet<char>> = rs
        .iter()
        .map(|x| x.0.keys().copied().chain(x.1.keys().copied()).collect())
        .collect();

    // Easier than writing a complex fold
    let common = &(&all[0] & &all[1]) & &all[2];

    common.into_iter().next().unwrap()
}

fn day_03_rucksack_reorganization() -> (i64, i64) {
    let in_fpath = "input/03.txt";
    let input = fs::read_to_string(in_fpath).unwrap();

    let rucksacks: Vec<Rucksack> = input.split('\n').map(parse_rucksack).collect();

    let part1_answer_misplaced_item = rucksacks
        .iter()
        .map(common_element)
        .map(element_priority)
        .sum();

    let elf_groups: Vec<Vec<Rucksack>> = rucksacks.chunks(3).map(|c| c.to_vec()).collect();
    let part2_answer_badge: i64 = elf_groups
        .iter()
        .map(|e| find_badge(e))
        .map(element_priority)
        .sum();

    (part1_answer_misplaced_item, part2_answer_badge)
}

fn main() {
    let (part1, part2) = day_03_rucksack_reorganization();
    println!("{}", part1);
    println!("{}", part2);
}
