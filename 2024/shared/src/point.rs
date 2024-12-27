use std::ops::Add;
use crate::grid_direction::{GridDirection, ALL_GRID_DIRECTIONS};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> [Self; 8] {
        ALL_GRID_DIRECTIONS.map(|d| self.translate(d))
    }

    pub fn direct_neighbors(&self) -> [Self; 4] {
        [
            GridDirection::U,
            GridDirection::R,
            GridDirection::D,
            GridDirection::L,
        ].map(|d| self.translate(d))
    }

    pub fn translate(&self, direction: GridDirection) -> Self {
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

    pub fn distance(&self, other: Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Point::new(1, 2);
        let b = Point::new(3, 4);

        let result = a + b;

        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn add_negative() {
        let a = Point::new(1, -2);
        let b = Point::new(-3, 4);

        let result = a + b;

        assert_eq!(result.x, -2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn neighbors() {
        let a = Point::new(4, 2);
        let result = a.neighbors();

        assert_eq!(result.len(), 8);
        assert!(result.contains(&Point::new(4, 1)));
        assert!(result.contains(&Point::new(5, 1)));
        assert!(result.contains(&Point::new(5, 2)));
        assert!(result.contains(&Point::new(5, 3)));
        assert!(result.contains(&Point::new(4, 3)));
        assert!(result.contains(&Point::new(3, 3)));
        assert!(result.contains(&Point::new(3, 2)));
        assert!(result.contains(&Point::new(3, 1)));
    }

    #[test]
    fn direct_neighbors() {
        let a = Point::new(4, 2);
        let result = a.direct_neighbors();

        assert_eq!(result.len(), 4);
        assert!(result.contains(&Point::new(4, 1)));
        assert!(result.contains(&Point::new(5, 2)));
        assert!(result.contains(&Point::new(4, 3)));
        assert!(result.contains(&Point::new(3, 2)));
    }

    #[test]
    fn translate_u() {
        let a = Point::new(4, 2);
        let result = a.translate(GridDirection::U);

        assert_eq!(result.x, 4);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn translate_ur() {
        let a = Point::new(4, 2);
        let result = a.translate(GridDirection::UR);

        assert_eq!(result.x, 5);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn translate_r() {
        let a = Point::new(4, 2);
        let result = a.translate(GridDirection::R);

        assert_eq!(result.x, 5);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn translate_dr() {
        let a = Point::new(4, 2);
        let result = a.translate(GridDirection::DR);

        assert_eq!(result.x, 5);
        assert_eq!(result.y, 3);
    }

    #[test]
    fn translate_d() {
        let a = Point::new(4, 2);
        let result = a.translate(GridDirection::D);

        assert_eq!(result.x, 4);
        assert_eq!(result.y, 3);
    }

    #[test]
    fn translate_dl() {
        let a = Point::new(4, 2);
        let result = a.translate(GridDirection::DL);

        assert_eq!(result.x, 3);
        assert_eq!(result.y, 3);
    }

    #[test]
    fn translate_l() {
        let a = Point::new(4, 2);
        let result = a.translate(GridDirection::L);

        assert_eq!(result.x, 3);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn translate_ul() {
        let a = Point::new(4, 2);
        let result = a.translate(GridDirection::UL);

        assert_eq!(result.x, 3);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn distance() {
        let a = Point::new(2, 3);

        assert_eq!(a.distance(Point::new(5, 1)), 5);
        assert_eq!(a.distance(Point::new(-1, -2)), 8);
    }
}
