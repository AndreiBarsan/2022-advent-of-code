use std::fs;

fn read_tree_map(in_fpath: &str) -> Vec<Vec<u8>> {
    std::fs::read_to_string(in_fpath)
        .expect("Could not read input file")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn print_tree_map(map: &Vec<Vec<u8>>) {
    for row in map {
        println!(
            "{}",
            row.iter()
                .map(|h| (*h + ('0' as u8)) as char)
                .collect::<String>()
        );
    }
}

fn day_08_treetop_tree_house() -> (usize, usize) {
    let in_fpath = "input/08.txt";

    let tree_map: Vec<Vec<u8>> = read_tree_map(in_fpath);
    // print_tree_map(&tree_map);

    // Run 'cum-max' operations over the four directions.
    // The clone is a hacky but functional way of getting arrays of the same shape.
    let mut north_max: Vec<Vec<u8>> = tree_map.clone();
    let mut south_max: Vec<Vec<u8>> = tree_map.clone();
    let mut east_max: Vec<Vec<u8>> = tree_map.clone();
    let mut west_max: Vec<Vec<u8>> = tree_map.clone();

    let mut cur_max = tree_map[0].clone();
    // Resetting the first row of north max is not really necessary, just handy for visualizing things.
    for cc in 0..tree_map[0].len() {
        north_max[0][cc] = 0;
    }
    for (row_idx, row) in tree_map.iter().enumerate() {
        if row_idx == 0 {
            continue;
        }
        for cc in 0..row.len() {
            north_max[row_idx][cc] = cur_max[cc];
            cur_max[cc] = cur_max[cc].max(row[cc]);
        }
    }

    for rr in 0..tree_map.len() {
        west_max[rr][0] = 0;
        cur_max[rr] = tree_map[rr][0];
    }
    for cc in 0..cur_max.len() {
        if cc == 0 {
            continue;
        }
        for rr in 0..tree_map.len() {
            west_max[rr][cc] = cur_max[rr];
            cur_max[rr] = cur_max[rr].max(tree_map[rr][cc]);
        }
    }

    for rr in 0..tree_map.len() {
        east_max[rr][cur_max.len() - 1] = 0;
        cur_max[rr] = tree_map[rr][cur_max.len() - 1];
    }
    for cc in (0..cur_max.len()).rev() {
        if cc == cur_max.len() - 1 {
            continue;
        }
        for rr in 0..tree_map.len() {
            east_max[rr][cc] = cur_max[rr];
            cur_max[rr] = cur_max[rr].max(tree_map[rr][cc]);
        }
    }

    for cc in 0..tree_map[0].len() {
        cur_max[cc] = south_max[tree_map.len() - 1][cc];
        south_max[tree_map.len() - 1][cc] = 0;
    }
    for (row_idx, row) in (tree_map.iter().enumerate()).rev() {
        if row_idx == tree_map.len() - 1 {
            continue;
        }
        for cc in 0..row.len() {
            south_max[row_idx][cc] = cur_max[cc];
            cur_max[cc] = cur_max[cc].max(row[cc]);
        }
    }

    let n_rows = tree_map.len();
    let n_cols = tree_map[0].len();
    let mut n_visible = 0;
    for rr in 1..(n_rows - 1) {
        for cc in 1..(n_cols - 1) {
            let cur = tree_map[rr][cc];
            if north_max[rr][cc] < cur
                || south_max[rr][cc] < cur
                || east_max[rr][cc] < cur
                || west_max[rr][cc] < cur
            {
                n_visible += 1;
            }
        }
    }
    n_visible += n_rows * 2 + n_cols * 2 - 4;

    // Part 2: Scenic Score
    //
    // For this one, I'll simply do a quadratic local search. Upon benchmarking, seems like even for the "contest"
    // input, this ugly solution takes maybe 0.2--0.3ms, so there's not a lot of reasons to optimize it.
    let mut max_scenic_score = 0;
    for rr in 0..n_rows {
        for cc in 0..n_cols {
            let cur = tree_map[rr][cc];

            let mut lc = 0;
            for left_idx in (0..cc).rev() {
                lc += 1;
                if tree_map[rr][left_idx] >= cur {
                    break;
                }
            }

            let mut rc = 0;
            for right_idx in (cc + 1)..n_cols {
                rc += 1;
                if tree_map[rr][right_idx] >= cur {
                    break;
                }
            }

            let mut uc = 0;
            for up_idx in (0..rr).rev() {
                uc += 1;
                if tree_map[up_idx][cc] >= cur {
                    break;
                }
            }

            let mut dc = 0;
            for down_idx in (rr + 1)..n_rows {
                dc += 1;
                if tree_map[down_idx][cc] >= cur {
                    break;
                }
            }

            let ss = lc * rc * uc * dc;
            if ss >= max_scenic_score {
                max_scenic_score = ss;
            }
        }
    }

    (n_visible, max_scenic_score)
}

fn main() {
    let (part1_answer, part2_answer) = day_08_treetop_tree_house();
    println!("{}", part1_answer);
    println!("{}", part2_answer);
}
