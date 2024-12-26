#![allow(dead_code)]

struct Coords {
    x: i32,
    y: i32,
}

pub enum GridDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
    LeftTopToRightBottom,
    RightBottomToLeftTop,
    LeftBottomToRightTop,
    RightTopToLeftBottom,
}

impl GridDirection {
    fn to_coords(&self) -> Coords {
        match self {
            GridDirection::LeftToRight => Coords { x: 1, y: 0 },
            GridDirection::RightToLeft => Coords { x: -1, y: 0 },
            GridDirection::TopToBottom => Coords { x: 0, y: 1 },
            GridDirection::BottomToTop => Coords { x: 0, y: -1 },
            GridDirection::LeftTopToRightBottom => Coords { x: 1, y: 1 },
            GridDirection::RightBottomToLeftTop => Coords { x: -1, y: -1 },
            GridDirection::LeftBottomToRightTop => Coords { x: 1, y: -1 },
            GridDirection::RightTopToLeftBottom => Coords { x: -1, y: 1 },
        }
    }
}

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

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        if self.in_bounds(x, y) {
            let index = self.index(x, y);
            Some(&self.data[index])
        } else {
            None
        }
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

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn iter(&self, from_x: i32, from_y: i32, direction: GridDirection) -> GridIterator<T> {
        if self.in_bounds(from_x, from_y) {
            let full_view = GridView {
                grid: self,
                grid_x: 0,
                grid_y: 0,
                width: self.width,
                height: self.height,
            };

            GridIterator {
                grid: full_view,
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
        if self.in_bounds(from_x, from_y) && self.in_bounds(from_x + width - 1, from_y + height - 1) {
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

    fn index(&self, x: i32, y: i32) -> usize {
        usize::try_from(y * self.width + x).unwrap()
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
            self.grid.get(self.grid_x + x, self.grid_y + y)
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
                grid: self.clone(),
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
    grid: GridView<'a, T>,
    current: Coords,
    change: Coords,
}

impl<'a, T: Clone> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.in_bounds(self.current.x, self.current.y) {
            let item = self.grid.get(self.current.x, self.current.y);
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
