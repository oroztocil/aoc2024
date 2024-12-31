#![allow(dead_code)]

use std::fmt;

#[derive(Clone)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum GridDirection {
    East,
    West,
    South,
    North,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl GridDirection {
    fn to_coords(&self) -> Coords {
        match self {
            GridDirection::East => Coords { x: 1, y: 0 },
            GridDirection::West => Coords { x: -1, y: 0 },
            GridDirection::South => Coords { x: 0, y: 1 },
            GridDirection::North => Coords { x: 0, y: -1 },
            GridDirection::NorthWest => Coords { x: -1, y: -1 },
            GridDirection::NorthEast => Coords { x: 1, y: -1 },
            GridDirection::SouthEast => Coords { x: 1, y: 1 },
            GridDirection::SouthWest => Coords { x: -1, y: 1 },
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    data: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn from_default(width: usize, height: usize, default_value: T) -> Self {
        check_dimensions(width, height);
        let item_count = usize::try_from(width * height).unwrap();

        Grid {
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            data: vec![default_value; item_count],
        }
    }

    pub fn from_lines(lines: Vec<Vec<T>>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        check_dimensions(width, height);

        let data = lines.concat();

        Grid {
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            data,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> &T {
        let index = self.index(x, y);
        &self.data[index]
    }

    pub fn try_get(&self, x: i32, y: i32) -> Option<&T> {
        if self.in_bounds(x, y) {
            let index = self.index(x, y);
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_by_coords(&self, coords: &Coords) -> &T {
        self.get(coords.x, coords.y)
    }

    pub fn set(&mut self, x: i32, y: i32, value: T) -> () {
        if self.in_bounds(x, y) {
            let index = self.index(x, y);
            self.data[index] = value
        } else {
            panic!(
                "Modifying grid with bounds ({0}, {1}) at ({x}, {y})",
                self.width, self.height
            )
        }
    }

    pub fn set_by_coords(&mut self, coords: &Coords, value: T) -> () {
        self.set(coords.x, coords.y, value);
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn try_move(&self, coords: &Coords, direction: &GridDirection) -> Option<Coords> {
        let change = direction.to_coords();
        let (new_x, new_y) = (coords.x + change.x, coords.y + change.y);
        if self.in_bounds(new_x, new_y) {
            Some(Coords { x: new_x, y: new_y })
        } else {
            None
        }
    }

    pub fn iter_all(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn enumerate_all(&self) -> GridEnumerator<T> {
        GridEnumerator {
            view: self.full_view(),
            current: 0
        }
    }

    pub fn iter(&self, from_x: i32, from_y: i32, direction: GridDirection) -> GridIterator<T> {
        if self.in_bounds(from_x, from_y) {
            GridIterator {
                view: self.full_view(),
                current: Coords {
                    x: from_x,
                    y: from_y,
                },
                change: direction.to_coords(),
            }
        } else {
            panic!(
                "Iterating grid with bounds ({0}, {1}) at ({from_x}, {from_y})",
                self.width, self.height
            )
        }
    }

    pub fn view(&self, from_x: i32, from_y: i32, width: i32, height: i32) -> GridView<T> {
        if self.in_bounds(from_x, from_y) && self.in_bounds(from_x + width - 1, from_y + height - 1)
        {
            GridView {
                grid: self,
                grid_x: from_x,
                grid_y: from_y,
                width: width,
                height: height,
            }
        } else {
            panic!(
                "Creating grid view at ({from_x}, {from_y}) with size ({width}, {height}) for grid with bounds ({0}, {1})",
                self.width, self.height
            )
        }
    }

    pub fn full_view(&self) -> GridView<T> {
        GridView {
            grid: self,
            grid_x: 0,
            grid_y: 0,
            width: self.width,
            height: self.height,
        }
    }

    fn index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                write!(f, "{}", self.data[idx])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn check_dimensions(width: usize, height: usize) {
    if width > i32::MAX.try_into().unwrap() {
        panic!("Invalid grid width {width}")
    }

    if height > i32::MAX.try_into().unwrap() {
        panic!("Invalid grid height {height}")
    }
}

#[derive(Clone)]
pub struct GridView<'a, T> {
    grid: &'a Grid<T>,
    grid_x: i32,
    grid_y: i32,
    pub width: i32,
    pub height: i32,
}

impl<'a, T: Clone> GridView<'a, T> {
    pub fn get(&self, x: i32, y: i32) -> Option<&'a T> {
        if self.in_bounds(x, y) {
            self.grid.try_get(self.grid_x + x, self.grid_y + y)
        } else {
            None
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn iter(&self, from_x: i32, from_y: i32, direction: GridDirection) -> GridIterator<T> {
        if self.in_bounds(from_x, from_y) {
            GridIterator {
                view: self.clone(),
                current: Coords {
                    x: from_x,
                    y: from_y,
                },
                change: direction.to_coords(),
            }
        } else {
            panic!(
                "Iterating grid with bounds ({0}, {1}) at ({from_x}, {from_y})",
                self.width, self.height
            )
        }
    }
}

pub struct GridIterator<'a, T> {
    view: GridView<'a, T>,
    current: Coords,
    change: Coords,
}

impl<'a, T: Clone> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.view.in_bounds(self.current.x, self.current.y) {
            let item = self.view.get(self.current.x, self.current.y);
            self.current = Coords {
                x: self.current.x + self.change.x,
                y: self.current.y + self.change.y,
            };
            item
        } else {
            None
        }
    }
}

pub struct GridEnumerator<'a, T> {
    view: GridView<'a, T>,
    current: usize
}

impl<'a, T: Clone> Iterator for GridEnumerator<'a, T> {
    type Item = (i32, i32, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= (self.view.width * self.view.height) as usize {
            return None;
        }
        
        let x = (self.current as i32) % self.view.width;
        let y = (self.current as i32) / self.view.width;

        if let Some(item) = self.view.get(x, y) {
            self.current += 1;
            Some((x, y, item))
        } else {
            None
        }
    }
}
