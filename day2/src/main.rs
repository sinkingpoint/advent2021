use libadvent::{must_read_input_to_lines, Position, Direction};

fn main() {
    let input = must_read_input_to_lines();
    let mut pos = Position::new(0, 0);

    // Part 1
    for line in input.iter() {
        let (dir, magnitude) = line.split_once(" ").unwrap();
        let magnitude = magnitude.parse().unwrap();
        let dir = match dir {
            "forward" => Direction::Right,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!()
        };

        pos.move_in_dir_with_magnitude(dir, magnitude);
    }

    println!("{}", pos.x * pos.y);

    let mut pos = Position::new(0, 0);
    let mut aim = 0;
    // Part 2
    for line in input.iter() {
        let (dir, magnitude) = line.split_once(" ").unwrap();
        let magnitude = magnitude.parse().unwrap();
        match dir {
            "forward" => {
                pos.move_in_dir_with_magnitude(Direction::Right, magnitude);
                pos.move_in_dir_with_magnitude(Direction::Down, magnitude * aim);
            },
            "up" => aim -= magnitude,
            "down" => aim += magnitude,
            _ => panic!()
        };
    }

    println!("{}", pos.x * pos.y);
}
