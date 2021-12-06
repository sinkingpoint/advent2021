use std::{ops::{Add, AddAssign, Mul, Sub, SubAssign}, fmt::Debug,str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

// Position represents a two dimensional location
pub struct Position<T> {
    pub x: T,
    pub y: T
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' | 'N' => Direction::Up,
            'D' | 'S' => Direction::Down,
            'L' | 'W' => Direction::Left,
            'R' | 'E' => Direction::Right,
            _ => panic!()
        }
    }
}

impl<T> Position<T> where T: Copy + PartialEq + PartialOrd + AddAssign + SubAssign + Add<Output=T> + Sub<Output=T> + Mul<Output=T> {
    pub fn new(x: T, y: T) -> Self {
        return Position{
            x,
            y
        }
    }

    pub fn new_from_str(s: &str) -> Self where T: FromStr, <T as FromStr>::Err: Debug {
        let (x, y) = s.split_once(",").map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap();
        return Self {
            x,
            y
        }
    }

    pub fn move_in_diru(&mut self, d: Direction) where T: From<u32> {
        self.move_in_dir_with_magnitude(d, 1.into());
    }

    pub fn move_in_dir(&mut self, d: Direction) where T: From<i32> {
        self.move_in_dir_with_magnitude(d, 1.into());
    }

    pub fn is_in_bounds_inclusive(&self, min: &Self, max: &Self) -> bool {
        return self.x >= min.x && self.y >= min.y && self.x <= max.x && self.y <= max.y
    }

    pub fn is_in_bounds_exclusive(&self, min: &Self, max: &Self) -> bool {
        return self.x >= min.x && self.y >= min.y && self.x < max.x && self.y < max.y
    }

    pub fn move_in_dir_with_magnitude(&mut self, d: Direction, m: T) {
        match &d {
            Direction::Down => self.y += m,
            Direction::Up => self.y -= m,
            Direction::Left => self.x -= m,
            Direction::Right => self.x += m,
        }
    }

    pub fn manhatten_distance(&self, other: &Self) -> T {
        let x = if self.x > other.x {
            self.x - other.x
        }
        else {
            other.x - self.x
        };

        let y = if self.y > other.y {
            self.y - other.y
        }
        else {
            other.y - self.y
        };

        return x + y;
    }

    pub fn distance_sq(&self, other: &Self) -> T {
        let deltax = other.x - self.x;
        let deltay = other.y - self.y;
        return deltax * deltax + deltay * deltay;
    }

    pub fn distance(&self, other: &Self) -> f64 where T: Into<f64> {
        let deltax = other.x - self.x;
        let deltay = other.y - self.y;
        return (deltax * deltax + deltay * deltay).into().sqrt();
    }
}

#[test]
fn test_position() {
    let mut position = Position::new(1, 1);
    position.move_in_dir(Direction::Up);
    position.move_in_dir_with_magnitude(Direction::Up, 5);

    assert_eq!(position.y, -5);
    assert_eq!(position.x, 1);

    assert_eq!(position.manhatten_distance(&Position::new(0, 0)), 6);
    assert_eq!(position.distance_sq(&Position::new(0, 0)), 26);

    position.move_in_dir_with_magnitude(Direction::Right, 2);
    position.move_in_dir_with_magnitude(Direction::Down, 2);
    assert_eq!(position.y, -3);
    assert_eq!(position.x, 3);
    assert_eq!(position.distance_sq(&Position::new(0, 0)), 18);
    assert_eq!(position.distance(&Position::new(0, 0)), (9. as f64 + 9.).sqrt());
}