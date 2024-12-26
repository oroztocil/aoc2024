pub mod grid;

use once_cell::sync::Lazy;
use std::{fs, path::PathBuf};

static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env!("OUT_DIR")));

pub fn read_input_file(relative_path: &str) -> String {
    fs::read_to_string(OUT_DIR.join(relative_path)).expect("Error reading file")
}

pub fn drop_element(input: &Vec<i32>, index: usize) -> Vec<i32> {
    input
        .iter()
        .take(index)
        .copied()
        .chain(input.iter().skip(index + 1).copied())
        .collect()
}
