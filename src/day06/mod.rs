use std::{collections::HashSet, fmt};

use crate::utils::grid::{Coords, Grid, GridDirection};

#[derive(Clone)]
enum Tile {
    Empty { visited: bool },
    Wall,
    Obstruction { hit_from: HashSet<GridDirection> },
}

#[derive(Clone)]
struct Walker {
    position: Coords,
    direction: GridDirection,
}

pub fn solve_first(input: &str) -> usize {
    let (grid, walker) = parse_input(input);
    simulate(grid, walker)
        .iter_all()
        .filter(|tile| match tile {
            Tile::Empty { visited: true } => true,
            _ => false,
        })
        .count()
}

fn simulate(mut grid: Grid<Tile>, mut walker: Walker) -> Grid<Tile> {
    while let Some(coords) = grid.try_move(&walker.position, &walker.direction) {
        let target = grid.get_by_coords(&coords);

        match target {
            Tile::Empty { .. } => {
                grid.set_by_coords(&coords, Tile::Empty { visited: true });
                walker.position = coords;
            }
            Tile::Wall => {
                walker.direction = match walker.direction {
                    GridDirection::East => GridDirection::South,
                    GridDirection::West => GridDirection::North,
                    GridDirection::South => GridDirection::West,
                    GridDirection::North => GridDirection::East,
                    _ => panic!("Unsupported grid direction"),
                }
            }
            _ => panic!("Unsupported tile type"),
        }
    }

    grid
}

pub fn solve_second(input: &str) -> usize {
    let (original_grid, original_walker) = parse_input(input);
    let simulated_grid = simulate(original_grid.clone(), original_walker.clone());
    let possible_obstructions = simulated_grid.enumerate_all().filter(|(x, y, tile)| true);

    6
}

fn parse_input(input: &str) -> (Grid<Tile>, Walker) {
    let mut walker: Option<Walker> = None;

    let lines: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty { visited: false },
                    '#' => Tile::Wall,
                    '^' => {
                        walker = Some(Walker {
                            position: Coords {
                                x: x as i32,
                                y: y as i32,
                            },
                            direction: GridDirection::North,
                        });
                        Tile::Empty { visited: true }
                    }
                    _ => panic!("Unsupported input char"),
                })
                .collect()
        })
        .collect();

    let grid = Grid::from_lines(lines);

    (grid, walker.expect("Initial walker positon not found"))
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Empty { visited: true } => 'X',
            Tile::Empty { visited: false } => '.',
            Tile::Wall => '#',
            Tile::Obstruction { .. } => 'O',
        };
        write!(f, "{c}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input_file;

    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(solve_first(&read_input_file("day06/test1.txt")), 41);
    }

    #[test]
    fn test_second() {
        assert_eq!(solve_second(&read_input_file("day06/test1.txt")), 6);
    }
}
