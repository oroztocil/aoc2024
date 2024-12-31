use std::{collections::HashSet, fmt};

use crate::utils::grid::{Coords, Grid, GridDirection};

#[derive(Clone)]
enum Tile {
    Empty { visited: bool },
    Wall { hit_from: HashSet<GridDirection> },
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
    simulate(grid, walker)
        .expect("Initial simulation stuck in loop")
        .iter_all()
        .filter(|tile| match tile {
            Tile::Empty { visited: true } => true,
            _ => false,
        })
        .count()
}

pub fn solve_second(input: &str) -> usize {
    let (original_grid, original_walker) = parse_input(input);

    // Get all visited non-start positions that candidates for obstruction placement
    let simulated_grid = simulate(original_grid.clone(), original_walker.clone())
        .expect("Initial simulation stuck in loop");

    let obstruction_candidates = simulated_grid
        .enumerate_all()
        .filter(|(x, y, tile)| match tile {
            Tile::Empty { visited: true } => {
                *x != original_walker.position.x || *y != original_walker.position.y
            }
            _ => false,
        });

    let mut result = 0;

    // Try each possible obstruction position
    // Run simulation until either walker leaves grid,
    // or hits the obstruction twice from the same direction
    for (x, y, _) in obstruction_candidates {
        let mut grid = original_grid.clone();
        grid.set(
            x,
            y,
            Tile::Wall {
                hit_from: HashSet::new(),
            },
        );

        match simulate(grid, original_walker.clone()) {
            Ok(_) => {}
            Err(LoopDetected) => result += 1,
        }
    }

    result
}

fn simulate(mut grid: Grid<Tile>, mut walker: Walker) -> Result<Grid<Tile>, LoopDetected> {
    while let Some(coords) = grid.try_move(&walker.position, &walker.direction) {
        let target = grid.get_by_coords(&coords);

        match target {
            Tile::Empty { .. } => {
                grid.set_by_coords(&coords, Tile::Empty { visited: true });
                walker.position = coords;
            }
            Tile::Wall { hit_from } => {
                if hit_from.contains(&walker.direction) {
                    return Err(LoopDetected);
                } else {
                    let mut updated_hit_from = hit_from.clone();
                    updated_hit_from.insert(walker.direction.clone());
                    grid.set_by_coords(
                        &coords,
                        Tile::Wall {
                            hit_from: updated_hit_from,
                        },
                    );
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

    Ok(grid)
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
                    '#' => Tile::Wall {
                        hit_from: HashSet::new(),
                    },
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
