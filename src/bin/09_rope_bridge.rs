use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

struct Rope {
    prev_head: Point,
    head: Point,
    tail: Point,
}

impl Rope {
    fn move_left(&mut self) {
        self.prev_head = self.head;
        self.head.x -= 1;
        if self.is_stretched() {
            self.tail = self.prev_head;
        }
    }

    fn move_right(&mut self) {
        self.prev_head = self.head;
        self.head.x += 1;
        if self.is_stretched() {
            self.tail = self.prev_head;
        }
    }

    fn move_up(&mut self) {
        self.prev_head = self.head;
        self.head.y += 1;
        if self.is_stretched() {
            self.tail = self.prev_head;
        }
    }

    fn move_down(&mut self) {
        self.prev_head = self.head;
        self.head.y -= 1;
        if self.is_stretched() {
            self.tail = self.prev_head;
        }
    }

    fn is_stretched(&self) -> bool {
        self.head.x.abs_diff(self.tail.x) > 1 || self.head.y.abs_diff(self.tail.y) > 1
    }
}

fn parse_line(line: &str) -> (Direction, usize) {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() != 2 {
        panic!("Invalid line: {}", line);
    }
    let dir = match parts[0] {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("Invalid line: {}", line),
    };
    let steps = parts[1].parse().expect("Invalid number");
    (dir, steps)
}

fn load_problem(in_fpath: &str) -> Vec<(Direction, usize)> {
    std::fs::read_to_string(in_fpath)
        .expect("Could not read input file.")
        .lines()
        .map(parse_line)
        .collect()
}

fn day_09_rope_bridge(in_fpath: &str) -> (usize, usize) {
    let actions = load_problem(in_fpath);
    // TODO(andrei): Is it faster to just use a 2D map?
    let mut tail_visited: HashSet<(i64, i64)> = HashSet::new();
    println!("{:?}", actions);

    let mut rope = Rope {
        prev_head: Point { x: 0, y: 0 },
        head: Point { x: 0, y: 0 },
        tail: Point { x: 0, y: 0 },
    };

    for (direction, n_steps) in actions {
        for _ in 0..n_steps {
            match direction {
                Direction::Left => rope.move_left(),
                Direction::Right => rope.move_right(),
                Direction::Up => rope.move_up(),
                Direction::Down => rope.move_down(),
            }
            tail_visited.insert((rope.tail.x, rope.tail.y));
        }
    }

    let n_unique_tail = tail_visited.len();

    (n_unique_tail, 0)
}

fn main() {
    let in_fpath = "input/09.txt";
    let (part1_answer, part2_answer) = day_09_rope_bridge(in_fpath);
    println!("{}", part1_answer);
    println!("{}", part2_answer);
}
