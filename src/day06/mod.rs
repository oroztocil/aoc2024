use std::{collections::HashSet, fmt};

use crate::utils::grid::{Coords, Grid, GridDirection};

#[derive(Clone)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Clone)]
struct Walker {
    position: Coords,
    direction: GridDirection,
}

#[derive(Debug)]
struct LoopDetected;

pub fn solve_first(input: &str) -> usize {
    let (grid, walker) = parse_input(input);
    let visited_coords = simulate(&grid, walker).expect("Initial simulation stuck in loop");
    visited_coords.len()
}

pub fn solve_second(input: &str) -> usize {
    let (mut grid, original_walker) = parse_input(input);

    // Get all visited non-start positions that candidates for obstruction placement
    let mut visited_coords = simulate(&grid, original_walker.clone())
        .expect("Initial simulation stuck in loop");

    visited_coords.remove(&original_walker.position);

    let mut result = 0;

    // Try each possible obstruction position
    // Run simulation until either walker leaves grid,
    // or hits the obstruction twice from the same direction
    for obstruction_coords in visited_coords {
        grid.set_by_coords(&obstruction_coords, Tile::Wall);

        match simulate(&grid, original_walker.clone()) {
            Ok(_) => {}
            Err(LoopDetected) => result += 1,
        }

        grid.set_by_coords(&obstruction_coords, Tile::Empty);
    }

    result
}

fn simulate(grid: &Grid<Tile>, mut walker: Walker) -> Result<HashSet<Coords>, LoopDetected> {
    let mut wall_hit_directions = HashSet::<(Coords, GridDirection)>::new();
    let mut visited_coords = HashSet::<Coords>::new();
    visited_coords.insert(walker.position.clone());

    while let Some(coords) = grid.try_move(&walker.position, &walker.direction) {
        let target = grid.get_by_coords(&coords);

        match target {
            Tile::Empty => {
                visited_coords.insert(coords.clone());
                walker.position = coords;
            }
            Tile::Wall => {
                let hit_direction = (coords, walker.direction.clone());
                if wall_hit_directions.contains(&hit_direction) {
                    return Err(LoopDetected);
                } else {
                    wall_hit_directions.insert(hit_direction);
                }

                walker.direction = match walker.direction {
                    GridDirection::East => GridDirection::South,
                    GridDirection::West => GridDirection::North,
                    GridDirection::South => GridDirection::West,
                    GridDirection::North => GridDirection::East,
                    _ => panic!("Unsupported grid direction"),
                }
            }
        }
    }

    Ok(visited_coords)
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
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    '^' => {
                        walker = Some(Walker {
                            position: Coords {
                                x: x as i32,
                                y: y as i32,
                            },
                            direction: GridDirection::North,
                        });
                        Tile::Empty
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
            Tile::Empty => '.',
            Tile::Wall { .. } => '#',
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
