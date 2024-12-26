use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let out_dir = env!("OUT_DIR");
    PathBuf::from(out_dir)
});


pub fn drop_element(input: &Vec<i32>, index: usize) -> Vec<i32> {
    input
        .iter()
        .take(index)
        .copied()
        .chain(input.iter().skip(index + 1).copied())
        .collect()
}
