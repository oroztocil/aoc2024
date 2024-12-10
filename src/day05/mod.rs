use std::{cmp::Ordering, collections::HashMap, fs};

pub fn solve_first(input_path: &str) -> usize {
    let (rule_map, updates) = load_input(input_path);
    let empty_rules = Vec::<u32>::new();

    let valid_updates = updates.iter().filter(|update| {
        for i in 0..update.len() - 1 {
            let left = update[i];
            let rules = rule_map.get(&left).unwrap_or(&empty_rules);
            for k in (i + 1)..update.len() {
                let right = update[k];
                if !rules.contains(&right) {
                    return false;
                }
            }
        }

        true
    });

    let mid_elements = valid_updates.map(|update| update[update.len() / 2]);
    mid_elements.sum::<u32>() as usize
}

pub fn solve_second(input_path: &str) -> usize {
    let (rule_map, updates) = load_input(input_path);
    let empty_rules = Vec::<u32>::new();

    let invalid_updates = updates.iter().filter(|update| {
        for i in 0..update.len() - 1 {
            let left = update[i];
            let rules = rule_map.get(&left).unwrap_or(&empty_rules);
            for k in (i + 1)..update.len() {
                let right = update[k];
                if !rules.contains(&right) {
                    return true;
                }
            }
        }

        false
    });

    let fixed_updates: Vec<Vec<u32>> = invalid_updates.map(|update| { 
        let mut fixed = update.clone();
        fixed.sort_by(|a, b| {
            let rules = rule_map.get(&a).unwrap_or(&empty_rules);
            if rules.contains(b) { Ordering::Less } else { Ordering::Greater }
        });
        fixed
    }).collect();

    let mid_elements = fixed_updates.iter().map(|update| update[update.len() / 2]);
    mid_elements.sum::<u32>() as usize
}

fn load_input(path: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let file_content = fs::read_to_string(path).expect("Error reading file");
    let (rule_part, update_part) = file_content.split_once("\n\n").unwrap();
    let rules = parse_rules(rule_part);
    let updates = parse_updates(update_part);

    (rules, updates)
}

fn parse_rules(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut rule_map = HashMap::new();

    for line in input.lines() {
        let (left, right) = line
            .split_once("|")
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unwrap();

        rule_map.entry(left).or_insert_with(Vec::new).push(right);
    }

    rule_map
}

fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    input
        .split_whitespace()
        .map(|line| line.split(",").map(|tok| tok.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first("C:/Dev/aoc2024/src/day05/test1.txt"), 143);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second("C:/Dev/aoc2024/src/day05/test1.txt"), 123);
    }
}
