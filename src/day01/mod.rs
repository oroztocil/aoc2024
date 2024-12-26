use std::{collections::HashMap, fs, path::Path};

pub fn solve_first(input_path: &Path) -> i32 {
    let (mut left, mut right) = get_nums(input_path);

    left.sort();
    right.sort();
    let mut total_diff = 0;

    for i in 0..left.len() {
        total_diff += (left[i] - right[i]).abs()
    }

    total_diff
}

pub fn solve_second(input_path: &Path) -> i32 {
    let (left, right) = get_nums(input_path);
    let mut histogram: HashMap<i32, i32> = HashMap::new();

    for num in right {
        *histogram.entry(num).or_insert(0) += 1;
    }

    let mut result = 0;

    for num in left {
        result += num * histogram.get(&num).cloned().unwrap_or(0)
    }

    result
}

fn get_nums(path: &Path) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(path).expect("Error reading input file");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }

    (left, right)
}