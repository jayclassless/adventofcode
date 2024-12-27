use std::io::BufRead;
use std::io::BufReader;
use crate::point::Point;


#[derive(Debug, Clone)]
pub struct Grid<T: Copy> {
    cells: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: Copy + PartialEq> Grid<T> {
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
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn chars_from_reader<R: BufRead>(reader: R) -> Grid<char> {
        let mut cells: Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let l: Vec<char> = line.expect("could not parse line").chars().collect();
            cells.push(l);
        }

        Self::new(cells)
    }

    pub fn chars_from_string(str: &str) -> Grid<char> {
        let reader = BufReader::new(str.as_bytes());
        Self::chars_from_reader(reader)
    }

    pub fn digits_from_reader<R: BufRead>(reader: R) -> Grid<u8> {
        let mut cells: Vec<Vec<u8>> = Vec::new();

        for line in reader.lines() {
            let l: Vec<u8> = line.expect("could not parse line").chars().map(|c| c.to_digit(10).expect("could not parse digit") as u8).collect();
            cells.push(l);
        }

        Self::new(cells)
    }

    pub fn empty_with_char(width: usize, height: usize, init: char) -> Grid<char> {
        let mut cells = Vec::new();

        for _ in 0..height {
            cells.push(Vec::from_iter(init.to_string().repeat(width).chars()));
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

    pub fn find(&self, target: T) -> Option<Point> {
        if let Some(search) = self.into_iter().find(|i| i.value == target) {
            return Some(search.point);
        }

        None
    }
}

impl<T: Copy + ToString> ToString for Grid<T> {
    fn to_string(&self) -> String {
       self.cells.iter()
            .map(|row| row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
            )
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub struct GridIter<'a, T: Copy> {
    grid: &'a Grid<T>,
    pos: Point,
}

#[derive(Debug, PartialEq)]
pub struct GridIteratorItem<T> {
    pub value: T,
    pub point: Point,
}

impl<'a, T: Copy + PartialEq> IntoIterator for &'a Grid<T> {
    type Item = GridIteratorItem<T>;
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            pos: Point::new(0, 0),
        }
    }
}

impl<'a, T: Copy + PartialEq> Iterator for GridIter<'a, T> {
    type Item = GridIteratorItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.pos;
        let value = self.grid.get_point(&self.pos);

        if value.is_none() {
            return None
        }

        let mut new_x = point.x;
        let mut new_y = point.y;
        if new_x == (self.grid.width - 1).try_into().unwrap() {
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
    fn find() {
        let a = Grid::<char>::new(test_input());

        let result = a.find('D');
        assert_eq!(result, Some(Point::new(0, 1)));

        let result = a.find('X');
        assert_eq!(result, None);
    }

    #[test]
    fn iterator() {
        let a = Grid::<char>::new(test_input());
        let mut iter = a.into_iter();

        let x = iter.next().unwrap();
        assert_eq!(x.value, 'A');
        assert_eq!(x.point, Point::new(0, 0));

        let x = iter.next().unwrap();
        assert_eq!(x.value, 'B');
        assert_eq!(x.point, Point::new(1, 0));

        let x = iter.next().unwrap();
        assert_eq!(x.value, 'C');
        assert_eq!(x.point, Point::new(2, 0));

        let x = iter.next().unwrap();
        assert_eq!(x.value, 'D');
        assert_eq!(x.point, Point::new(0, 1));

        let x = iter.next().unwrap();
        assert_eq!(x.value, 'E');
        assert_eq!(x.point, Point::new(1, 1));

        let x = iter.next().unwrap();
        assert_eq!(x.value, 'F');
        assert_eq!(x.point, Point::new(2, 1));

        let x = iter.next().unwrap();
        assert_eq!(x.value, 'G');
        assert_eq!(x.point, Point::new(0, 2));

        let x = iter.next().unwrap();
        assert_eq!(x.value, 'H');
        assert_eq!(x.point, Point::new(1, 2));

        let x = iter.next().unwrap();
        assert_eq!(x.value, 'I');
        assert_eq!(x.point, Point::new(2, 2));

        let x = iter.next();
        assert_eq!(x, None);
    }
    
    #[test]
    fn to_string() {
        let a = Grid::<char>::new(test_input());

        assert_eq!(a.to_string(), "ABC\nDEF\nGHI");
    }
}
