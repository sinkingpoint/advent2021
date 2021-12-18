use std::collections::HashSet;

use libadvent::*;

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32)
}

// i = 3
// x = 5
// 3 + (5 - 3)

fn fold(dots: &Vec<Position<i32>>, fold: &Fold) -> Vec<Position<i32>>{
    let mut new_dots = HashSet::new();

    for pos in dots {
        let (x, y) = match fold {
            Fold::X(i) => {
                if i > &pos.x {
                    (pos.x, pos.y)
                }
                else {
                    (i + (i - pos.x), pos.y)
                }
            },
            Fold::Y(i) => {
                if i > &pos.y {
                    (pos.x, pos.y)
                }
                else {
                    (pos.x, i + (i - pos.y))
                }
            },
        };

        new_dots.insert(Position::new(x, y));
    }

    return new_dots.into_iter().collect();
}

fn print(dots: &Vec<Position<i32>>) {
    let mut output = String::new();
    let min_x = dots.iter().min_by_key(|p| p.x).unwrap().x;
    let min_y = dots.iter().min_by_key(|p| p.y).unwrap().y;

    let max_x = dots.iter().max_by_key(|p| p.x).unwrap().x;
    let max_y = dots.iter().max_by_key(|p| p.y).unwrap().y;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if dots.contains(&Position::new(x, y)) {
                output.push('█');
            }
            else {
                output.push(' ');
            }
        }

        output.push('\n');
    }

    println!("{}", output)
}

fn main() {
    let input = must_read_input_to_lines();

    let mut dots = Vec::new();
    for line in input.iter().filter(|s| s.contains(",")) {
        let (x, y) = line.split_once(",").unwrap();
        dots.push(Position::<i32>::new(x.parse().unwrap(), y.parse().unwrap()));
    }

    let mut folds = Vec::new();
    for line in input.iter().filter(|s| s.starts_with("fold along")) {
        let end = line.split(" ").last().unwrap();
        let (axis, amt) = end.split_once("=").unwrap();

        let fold = match axis {
            "x" => Fold::X(amt.parse().unwrap()),
            "y" => Fold::Y(amt.parse().unwrap()),
            _ => panic!()
        };

        folds.push(fold);
    }

    dots = fold(&dots, &folds[0]);
    println!("{} dots after one fold", dots.len());

    for i in 1..folds.len() {
        dots = fold(&dots, &folds[i]);
    }

    print(&dots);
}

