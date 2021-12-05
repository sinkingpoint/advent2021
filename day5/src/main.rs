use std::hash::Hash;

use libadvent::*;

#[derive(Debug, Hash)]
struct Line {
    start: Position<i32>,
    end: Position<i32>
}

impl Line {
    fn new_from_str(s: &str) -> Self {
        let (start, end) = s.split_once(" -> ").unwrap();

        return Self {
            start: Position::new_from_str(start),
            end: Position::new_from_str(end)
        }
    }

    fn get_points(&self) -> Vec<Position<i32>> {
        let diff_x = self.end.x - self.start.x;
        let diff_y = self.end.y - self.start.y;

        let steps = diff_x.abs().max(diff_y.abs());
        let grad_x = diff_x / steps;
        let grad_y = diff_y / steps;

        let mut current_x = self.start.x;
        let mut current_y = self.start.y;

        let mut out = Vec::new();
        while current_x != self.end.x || current_y != self.end.y {
            out.push(Position::new(current_x, current_y));
            current_x += grad_x;
            current_y += grad_y;
        }

        out.push(Position::new(current_x, current_y));

        return out;
    }
}

fn main() {
    // Part 1
    let input: Vec<Line> = must_read_input_to_lines().iter().map(|s| Line::new_from_str(s)).collect();

    let straights: Vec<Position<i32>> = input.iter().filter(|l| l.start.x == l.end.x || l.start.y == l.end.y).map(|i| i.get_points()).flatten().collect();
    let counts = count_occurences(straights.into_iter());

    let overlap_counts = counts.iter().filter(|(_, &c)| c > 1).count();
    println!("{} overlapping positions", overlap_counts);

    // Part 2
    let all_points: Vec<Position<i32>> = input.iter().map(|l| l.get_points()).flatten().collect();
    let counts = count_occurences(all_points.into_iter());

    let overlap_counts = counts.iter().filter(|(_, &c)| c > 1).count();
    println!("{} overlapping positions", overlap_counts);
}
