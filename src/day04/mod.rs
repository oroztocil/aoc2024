use crate::utils::grid::{Grid, GridDirection};

pub fn solve_first(input: &str) -> usize {
    let grid = parse_input(input);

    let mut passes = vec![
        // Corner passes
        grid.iter(0, 0, GridDirection::East),
        grid.iter(grid.width - 1, 0, GridDirection::West),
        grid.iter(0, grid.height - 1, GridDirection::East),
        grid.iter(grid.width - 1, grid.height - 1, GridDirection::West),
    ];

    // Left column passes
    for y in 1..grid.height - 1 {
        passes.push(grid.iter(0, y, GridDirection::East));
        passes.push(grid.iter(0, y, GridDirection::SouthEast));
        passes.push(grid.iter(0, y, GridDirection::NorthEast));
    }

    // Right column passes
    for y in 1..grid.height - 1 {
        passes.push(grid.iter(grid.width - 1, y, GridDirection::West));
        passes.push(grid.iter(grid.width - 1, y, GridDirection::NorthWest));
        passes.push(grid.iter(grid.width - 1, y, GridDirection::SouthWest));
    }

    // Top row passes
    for x in 0..grid.width {
        passes.push(grid.iter(x, 0, GridDirection::South));
        passes.push(grid.iter(x, 0, GridDirection::SouthEast));
        passes.push(grid.iter(x, 0, GridDirection::SouthWest));
    }

    // Bottom row passes
    for x in 0..grid.width {
        passes.push(grid.iter(x, grid.height - 1, GridDirection::North));
        passes.push(grid.iter(x, grid.height - 1, GridDirection::NorthWest));
        passes.push(grid.iter(x, grid.height - 1, GridDirection::NorthEast));
    }

    passes
        .into_iter()
        .map(|pass| pass.collect::<String>().matches("XMAS").count())
        .sum()
}

pub fn solve_second(input: &str) -> usize {
    let grid = parse_input(input);
    let mut result = 0;

    for x in 0..grid.width - 2 {
        for y in 0..grid.height - 2 {
            let view = grid.view(x, y, 3, 3);
            let passes = [
                view.iter(0, 0, GridDirection::SouthEast),
                view.iter(2, 2, GridDirection::NorthWest),
                view.iter(0, 2, GridDirection::NorthEast),
                view.iter(2, 0, GridDirection::SouthWest),
            ];

            if passes
                .into_iter()
                .map(|pass| pass.collect::<String>().matches("MAS").count())
                .sum::<usize>()
                == 2
            {
                result += 1;
            }
        }
    }

    result
}

fn parse_input(input: &str) -> Grid<char> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    Grid::from_lines(lines)
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input_file;

    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first(&read_input_file("day04/test1.txt")), 18);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second(&read_input_file("day04/test2.txt")), 9);
    }
}
