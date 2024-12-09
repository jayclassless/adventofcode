use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn neighbors(self) -> Vec<Self> {
        let mut points = Vec::new();
    
        for direction in ALL_GRID_DIRECTIONS {
            if let Some(point) = self.translate(direction) {
                points.push(point);
            }
        }

        points
    }

    pub fn translate(self, direction: GridDirection) -> Option<Self> {
        let mut new_x = self.x;
        let mut new_y = self.y;

        match direction {
            GridDirection::U => {
                if let Some(new) = new_y.checked_sub(1) {
                    new_y = new
                } else {
                    return None
                }
            },
            GridDirection::UR => {
                new_x += 1;
                if let Some(new) = new_y.checked_sub(1) {
                    new_y = new
                } else {
                    return None
                }
            },
            GridDirection::R => { new_x += 1 },
            GridDirection::DR => { new_y += 1; new_x += 1 },
            GridDirection::D => { new_y += 1 },
            GridDirection::DL => {
                new_y += 1;
                if let Some(new) = new_x.checked_sub(1) {
                    new_x = new
                } else {
                    return None
                }
            },
            GridDirection::L => {
                if let Some(new) = new_x.checked_sub(1) {
                    new_x = new
                } else {
                    return None
                }
            },
            GridDirection::UL => {
                if let Some(new) = new_y.checked_sub(1) {
                    new_y = new
                } else {
                    return None
                }

                if let Some(new) = new_x.checked_sub(1) {
                    new_x = new
                } else {
                    return None
                }
            },
        }

        Some(Self::new(new_x, new_y))
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
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


pub struct Grid<T> {
    cells: Vec<Vec<T>>
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new()
        }
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        if let Some(row) = self.cells.get(point.y) {
            return row.get(point.x);
        }

        None
    }
}
