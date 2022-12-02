// Day 01: Calorie Counting
//
// Main lesson: 'Reverse' wrapper to quickly reverse ordering for functions which need comparators.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

fn day_01_calorie_counting() -> (i64, i64) {
    let input_fname = "input/01.txt";
    let data: String = fs::read_to_string(input_fname).expect("Unable to read file.");

    // Variables for part two. Overkill solution with a min-heap.
    let ktop: usize = 3;
    let mut top = BinaryHeap::new();

    // Seems like doing this problem with a 'group_by' is possible but awkward and not very pragmatic.
    let mut max_calories: i64 = -1;
    let mut cur_calories: i64 = -2;

    for entry in data.lines() {
        if entry.len() == 0 {
            if cur_calories > max_calories {
                max_calories = cur_calories;
            }
            if cur_calories > 0 {
                match top.len() {
                    l if l < ktop => top.push(Reverse(cur_calories)),
                    _ => {
                        let third = top.peek().unwrap().0;
                        if third < cur_calories {
                            top.push(Reverse(cur_calories));
                            top.pop();
                        }
                    }
                }
            }
            cur_calories = 0;
        } else {
            cur_calories += i64::from_str_radix(entry, 10).unwrap();
        }
    }

    if cur_calories > max_calories {
        max_calories = cur_calories;
    }

    let top_three_cal_sum = top.into_iter().map(|x| x.0).sum::<i64>();

    (max_calories, top_three_cal_sum)
}

fn main() {
    println!("{:?}", day_01_calorie_counting());
}
