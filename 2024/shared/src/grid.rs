use std::io::BufRead;
use crate::point::Point;


#[derive(Debug)]
pub struct Grid<T: Copy> {
    cells: Vec<Vec<T>>,
    width: usize,
    height: usize,
    pos: Point,
}

impl<T: Copy> Grid<T> {
    pub fn new<X: Copy>(cells: Vec<Vec<X>>) -> Grid<X> {
        let height = cells.len();
        let width = if height > 0 { cells[0].len() } else { 0 };

        for row in cells.iter() {
            if row.len() != width {
                panic!("grid width is not uniform")
            }
        }

        Grid {
            cells,
            width,
            height,
            pos: Point::new(0, 0),
        }
    }

    pub fn chars_from_reader<R: BufRead>(reader: R) -> Grid<char> {
        let mut cells: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let l: Vec<char> = line.expect("could not parse line").chars().collect();
            cells.push(l);
        }

        Self::new(cells)
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn point_in_bounds(&self, point: &Point) -> bool {
        if point.x >= 0 && point.y >= 0 {
            return self.in_bounds(point.x as usize, point.y as usize);
        }

        false
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if let Some(row) = self.cells.get(y) {
            if let Some(cell) = row.get(x) {
                return Some(*cell)
            }
        }

        None
    }

    pub fn get_point(&self, point: &Point) -> Option<T> {
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

    pub fn set_point(&mut self, point: &Point, value: T) -> Option<Point> {
        if !self.point_in_bounds(point) {
            return None;
        }

        self.set(point.x as usize, point.y as usize, value)
    }
}

#[derive(Debug, PartialEq)]
pub struct GridIteratorItem<T> {
    pub value: T,
    pub point: Point,
}

impl<T: Copy> Iterator for Grid<T> {
    type Item = GridIteratorItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.pos;
        let value = self.get_point(&self.pos);

        if value.is_none() {
            self.pos = Point::new(0, 0);
            return None
        }

        let mut new_x = point.x;
        let mut new_y = point.y;
        if new_x == (self.width - 1).try_into().unwrap() {
            new_x = 0;
            new_y += 1;
        } else {
            new_x += 1;
        }
        self.pos = Point::new(new_x, new_y);

        Some(GridIteratorItem { value: value.unwrap(), point })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<Vec<char>> {
        return Vec::from([
            Vec::from(['A','B','C']),
            Vec::from(['D','E','F']),
            Vec::from(['G','H','I']),
        ]);
    }

    #[test]
    fn in_bounds() {
        let a = Grid::<char>::new(test_input());

        assert!(a.in_bounds(0, 0));
        assert!(a.in_bounds(2, 2));

        assert!(!a.in_bounds(7, 0));
        assert!(!a.in_bounds(2, 7));
    }

    #[test]
    fn point_in_bounds() {
        let a = Grid::<char>::new(test_input());

        assert!(a.point_in_bounds(&Point::new(0, 0)));
        assert!(a.point_in_bounds(&Point::new(2, 2)));

        assert!(!a.point_in_bounds(&Point::new(7, 0)));
        assert!(!a.point_in_bounds(&Point::new(2, 7)));
        assert!(!a.point_in_bounds(&Point::new(-1, 1)));
        assert!(!a.point_in_bounds(&Point::new(1, -1)));
    }

    #[test]
    fn get() {
        let a = Grid::<char>::new(test_input());

        assert_eq!(a.get(0, 0), Some('A'));
        assert_eq!(a.get(2, 2), Some('I'));
        assert_eq!(a.get(7, 2), None);
        assert_eq!(a.get(2, 7), None);
    }

    #[test]
    fn get_point() {
        let a = Grid::<char>::new(test_input());

        assert_eq!(a.get_point(&Point::new(0, 0)), Some('A'));
        assert_eq!(a.get_point(&Point::new(2, 2)), Some('I'));
        assert_eq!(a.get_point(&Point::new(7, 2)), None);
        assert_eq!(a.get_point(&Point::new(2, 7)), None);
        assert_eq!(a.get_point(&Point::new(-1, 2)), None);
        assert_eq!(a.get_point(&Point::new(2, -1)), None);
    }

    #[test]
    fn set() {
        let mut a = Grid::<char>::new(test_input());

        let result = a.set(0, 0, 'X');
        assert_eq!(a.get(0, 0), Some('X'));
        assert_eq!(result, Some(Point::new(0, 0)));

        let result = a.set(7, 0, 'X');
        assert_eq!(result, None);
    }

    #[test]
    fn set_point() {
        let mut a = Grid::<char>::new(test_input());

        let result = a.set_point(&Point::new(0, 0), 'X');
        assert_eq!(a.get(0, 0), Some('X'));
        assert_eq!(result, Some(Point::new(0, 0)));

        let result = a.set_point(&Point::new(7, 0), 'X');
        assert_eq!(result, None);

        let result = a.set_point(&Point::new(-1, 0), 'X');
        assert_eq!(result, None);
    }

    #[test]
    fn iterator() {
        let mut a = Grid::<char>::new(test_input());

        let x = a.next().unwrap();
        assert_eq!(x.value, 'A');
        assert_eq!(x.point, Point::new(0, 0));

        let x = a.next().unwrap();
        assert_eq!(x.value, 'B');
        assert_eq!(x.point, Point::new(1, 0));

        let x = a.next().unwrap();
        assert_eq!(x.value, 'C');
        assert_eq!(x.point, Point::new(2, 0));

        let x = a.next().unwrap();
        assert_eq!(x.value, 'D');
        assert_eq!(x.point, Point::new(0, 1));

        let x = a.next().unwrap();
        assert_eq!(x.value, 'E');
        assert_eq!(x.point, Point::new(1, 1));

        let x = a.next().unwrap();
        assert_eq!(x.value, 'F');
        assert_eq!(x.point, Point::new(2, 1));

        let x = a.next().unwrap();
        assert_eq!(x.value, 'G');
        assert_eq!(x.point, Point::new(0, 2));

        let x = a.next().unwrap();
        assert_eq!(x.value, 'H');
        assert_eq!(x.point, Point::new(1, 2));

        let x = a.next().unwrap();
        assert_eq!(x.value, 'I');
        assert_eq!(x.point, Point::new(2, 2));

        let x = a.next();
        assert_eq!(x, None);
    }
}
