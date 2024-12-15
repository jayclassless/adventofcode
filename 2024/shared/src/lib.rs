use std::io::BufRead;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn neighbors(self) -> [Self; 8] {
        ALL_GRID_DIRECTIONS.map(|d| self.translate(d))
    }

    pub fn translate(self, direction: GridDirection) -> Self {
        match direction {
            GridDirection::U => {
                Self::new(self.x, self.y - 1)
            },
            GridDirection::UR => {
                Self::new(self.x + 1, self.y - 1)
            },
            GridDirection::R => {
                Self::new(self.x + 1, self.y)
            },
            GridDirection::DR => {
                Self::new(self.x + 1, self.y + 1)
            },
            GridDirection::D => {
                Self::new(self.x, self.y + 1)
            },
            GridDirection::DL => {
                Self::new(self.x - 1, self.y + 1)
            },
            GridDirection::L => {
                Self::new(self.x - 1, self.y)
            },
            GridDirection::UL => {
                Self::new(self.x - 1, self.y - 1)
            },
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}


pub enum GridDirection {
    U,
    UR,
    R,
    DR,
    D,
    DL,
    L,
    UL,
}

const ALL_GRID_DIRECTIONS: [GridDirection; 8] = [
    GridDirection::U,
    GridDirection::UR,
    GridDirection::R,
    GridDirection::DR,
    GridDirection::D,
    GridDirection::DL,
    GridDirection::L,
    GridDirection::UL,
];


#[derive(Debug)]
pub struct Grid<T> {
    size: usize,
    cells: Vec<Vec<T>>,
    // pos: Point,
}

impl<T> Grid<T> {
    pub fn chars_from_file<R: BufRead>(reader: R) -> Grid<char> {
        let mut cells: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let l: Vec<char> = line.expect("could not parse line").chars().collect();
            cells.push(l);
        }

        let size = cells[0].len();

        if cells.len() != size {
            panic!("grid is not square")
        }

        for row in cells.iter() {
            if row.len() != size {
                panic!("grid size is not uniform")
            }
        }

        Grid {
            size,
            cells,
            // pos: Point::new(0, 0),
        }
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }

    pub fn point_in_bounds(&self, point: Point) -> bool {
        if point.x >= 0 && point.y >= 0 {
            return self.in_bounds(point.x as usize, point.y as usize);
        }

        false
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if let Some(row) = self.cells.get(y) {
            return row.get(x);
        }

        None
    }

    pub fn get_point(&self, point: Point) -> Option<&T> {
        if !self.point_in_bounds(point) {
            return None;
        }

        self.get(point.x as usize, point.y as usize)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<Point> {
        if !self.in_bounds(x, y) {
            return None;
        }

        self.cells[y][x] = value;
        Some(Point::new(x as isize, y as isize))
    }

    pub fn set_point(&mut self, point: Point, value: T) -> Option<Point> {
        if !self.point_in_bounds(point) {
            return None;
        }

        self.set(point.x as usize, point.y as usize, value)
    }
}

// pub struct GridIteratorItem<T> {
//     value: &T,
//     point: Point,
// }

// impl<T> Iterator for Grid<T> {
//     type Item = GridIteratorItem<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let point = self.pos;
//         let value = self.get_point(self.pos).unwrap();


//         Some(GridIteratorItem { value, point })
//     }
// }