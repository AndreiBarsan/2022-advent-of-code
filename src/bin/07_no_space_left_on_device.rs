use std::collections::HashSet;
use std::fs;

/// Parses a file tree from synthetic input 'ls' and 'cd' outputs.
///
fn parse_in_file(fpath: &str) -> Vec<(Vec<String>, usize)> {
    let mut cwd: Vec<String> = Vec::new();
    let mut all_paths: Vec<(Vec<String>, usize)> = Vec::new();

    let lines: Vec<String> = fs::read_to_string(fpath)
        .expect("Could not read input file")
        .lines()
        .map(|x| x.to_string())
        .collect();

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "$" {
            if parts[1] == "ls" {
                // no-op - for ls we care about the output lines!
            } else if parts[1] == "cd" {
                let arg = parts[2];
                if arg == "/" {
                    cwd.clear();
                } else if arg == ".." {
                    cwd.pop();
                } else {
                    cwd.push(arg.to_string());
                }
            } else {
                panic!("Invalid command: {}", parts[0]);
            }
        } else {
            // A 'ls' output line which will tell us about a new  entry in our file system.
            let lhs = parts[0];
            let rhs = parts[1];
            let mut full_path = cwd.clone();
            full_path.push(rhs.to_string());

            if lhs == "dir" {
                all_paths.push((full_path, 0));
            } else {
                let sz: usize = lhs.parse().unwrap();
                all_paths.push((full_path, sz));
            }
        }
    }

    all_paths
}

const MAX_SIZE: usize = 100000;

fn day_07_no_space_left_on_device() -> (usize, usize) {
    let in_fpath = "input/07.txt";
    let root = parse_in_file(in_fpath);

    // My Rust code originally failed to count '41223 rwr.nbz'.
    // This was because there was also a file whose name is a prefix of the above file's parent folder.
    // The fix was to add the `+ "/"` when constructing dirname prefixes below!

    let unique_dirs: HashSet<String> = root
        .iter()
        .filter(|x| x.1 == 0)
        .map(|x| x.0.join("/") + "/")
        .collect();

    let mut sum_of_small_dirs: usize = 0;

    let mut candidates: Vec<usize> = Vec::new();
    let used: usize = root.iter().map(|x| x.1).sum();
    let total = 70000000;
    let goal = 30000000;
    let free = total - used;
    let to_clear = goal - free;

    for dir_path in unique_dirs {
        if dir_path.len() == 0 {
            continue;
        }
        let mut dir_sum = 0;
        for (file_path, size) in &root {
            let cp = file_path.join("/");
            if cp.starts_with(&dir_path) {
                dir_sum += size;
            }
        }
        if dir_sum <= MAX_SIZE {
            sum_of_small_dirs += dir_sum;
        }
        if dir_sum > to_clear {
            candidates.push(dir_sum);
        }
    }

    (
        sum_of_small_dirs,
        *candidates
            .iter()
            .reduce(|acc, el| if el < acc { el } else { acc })
            .unwrap(),
    )
}

fn main() {
    let (part1_answer, part2_answer) = day_07_no_space_left_on_device();
    println!("{}", part1_answer);
    println!("{}", part2_answer);
}
