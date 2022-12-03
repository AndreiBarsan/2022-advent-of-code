use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

fn move_points(mv: &Move) -> i64 {
    match mv {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3,
    }
}

fn score(other: &Move, own: &Move) -> i64 {
    let game_score = match (other, own) {
        (Move::Rock, Move::Rock) => 3,
        (Move::Rock, Move::Paper) => 6,
        (Move::Rock, Move::Scissor) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Paper, Move::Paper) => 3,
        (Move::Paper, Move::Scissor) => 6,
        (Move::Scissor, Move::Rock) => 6,
        (Move::Scissor, Move::Paper) => 0,
        (Move::Scissor, Move::Scissor) => 3,
    };
    move_points(own) + game_score
}

fn parse_innocent_line(s: &str) -> (Move, Move) {
    let cc: Vec<char> = s.chars().collect();
    let lhs = match cc[0] {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissor,
        _ => panic!(),
    };
    let rhs = match cc[2] {
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissor,
        _ => panic!(),
    };

    (lhs, rhs)
}

fn parse_fixed_match_line(s: &str) -> (Move, Move) {
    let cc: Vec<char> = s.chars().collect();
    let lhs = match cc[0] {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissor,
        _ => panic!(),
    };
    let rhs = match cc[2] {
        // X means we have to lose
        'X' => match lhs {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
        },
        // Y means we need a draw
        'Y' => lhs.clone(),
        // Z means we need to win
        'Z' => match lhs {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        },
        _ => panic!(),
    };
    (lhs, rhs)
}

fn day_02_rock_paper_scissors() -> (i64, i64) {
    let in_fpath = "input/02.txt";
    let input = fs::read_to_string(in_fpath).unwrap();

    let part1_answer = input
        .split('\n')
        .map(parse_innocent_line)
        .map(|(l, r)| score(&l, &r))
        .sum();

    let part2_answer = input
        .split('\n')
        .map(parse_fixed_match_line)
        .map(|(l, r)| score(&l, &r))
        .sum();

    (part1_answer, part2_answer)
}

fn main() {
    let (part1, part2) = day_02_rock_paper_scissors();
    println!("{}", part1);
    println!("{}", part2);
}
