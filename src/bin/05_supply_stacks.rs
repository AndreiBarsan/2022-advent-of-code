use std::fs;

#[derive(Debug, Clone)]
struct Shipyard {
    crates: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Command {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_state(lines: &[String]) -> Shipyard {
    let mut crates: Vec<Vec<char>> = Vec::new();
    let block_width: usize = 4;

    for line in lines {
        for (cur_prefix_len, ch) in line.chars().enumerate() {
            if ch == ' ' || ch == '[' || ch == ']' {
                // No-op
            } else if ('0'..='9').contains(&ch) {
                // The last line, we don't need it since it's just the sequential indexes
                break;
            } else if ('A'..='Z').contains(&ch) {
                let idx = cur_prefix_len / block_width;

                while crates.len() <= idx {
                    crates.push(Vec::new());
                }
                // Safe to unwrap, as the loop above guarantees we'll have enough sub-vectors.
                crates.get_mut(idx).unwrap().push(ch);
            } else {
                panic!("Invalid character {} in line {}", ch, line);
            }
        }
    }
    Shipyard {
        crates: crates
            .iter()
            .map(|v| v.iter().rev().copied().collect())
            .collect(),
    }
}

fn parse_command(raw_str: &str) -> Command {
    let ss: Vec<&str> = raw_str.split(' ').collect();
    // TODO(andrei): Would be nice to use a regex with capture groups to avoid the ugly unwraps.
    Command {
        from: ss[3].parse().unwrap(),
        to: ss[5].parse().unwrap(),
        count: ss[1].parse().unwrap(),
    }
}

fn parse_commands(lines: &[String]) -> Vec<Command> {
    lines.iter().map(|x| parse_command(x)).collect()
}

fn execute_v9000(command: &Command, shipyard: &mut Shipyard) {
    for _ in 0..command.count {
        let moved_crate = shipyard.crates[command.from - 1usize].pop().unwrap();
        shipyard
            .crates
            .get_mut(command.to - 1usize)
            .unwrap()
            .push(moved_crate);
    }
}

/// Part 2 Logic: Same as v9000, except grabs all 'count' crates at once.
fn execute_v9001(command: &Command, shipyard: &mut Shipyard) {
    let mut batch: Vec<char> = Vec::new();
    for _ in 0..command.count {
        let moved_crate = shipyard.crates[command.from - 1usize].pop().unwrap();
        batch.push(moved_crate);
    }

    let mut rev_batch: Vec<char> = batch.iter().rev().copied().collect();
    shipyard
        .crates
        .get_mut(command.to - 1)
        .unwrap()
        .append(&mut rev_batch);
}

fn report(shipyard: &Shipyard) -> String {
    shipyard
        .crates
        .iter()
        .flat_map(|stack| stack.last())
        .collect()
}

fn day_05_supply_stacks() -> (String, String) {
    let in_fpath = "input/05.txt";
    let input = fs::read_to_string(in_fpath).expect("Could not read input file!");

    let start_state_lines: Vec<String> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();
    let command_lines: Vec<String> = input
        .lines()
        .skip(start_state_lines.len() + 1)
        .map(|x| x.to_string())
        .collect();

    let mut shipyard_part1 = parse_state(&start_state_lines);
    let mut shipyard_part2 = shipyard_part1.clone();
    let commands = parse_commands(&command_lines);

    for command in &commands {
        execute_v9000(command, &mut shipyard_part1);
    }

    for command in &commands {
        execute_v9001(command, &mut shipyard_part2);
    }

    (report(&shipyard_part1), report(&shipyard_part2))
}

fn main() {
    let (part1_answer, part2_answer) = day_05_supply_stacks();
    println!("{}", part1_answer);
    println!("{}", part2_answer);
}
