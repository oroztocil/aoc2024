pub fn drop_element(input: &Vec<i32>, index: usize) -> Vec<i32> {
    input
        .iter()
        .take(index)
        .copied()
        .chain(input.iter().skip(index + 1).copied())
        .collect()
}
