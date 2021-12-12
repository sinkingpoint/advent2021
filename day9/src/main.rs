use std::collections::HashSet;

use libadvent::*;

fn get_adjacents(map: &Vec<Vec<u8>>, pos: Position<i32>) -> Vec<u8> {
    let mut surrounds = Vec::new();
    for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
        let mut new_position = pos.clone();
        new_position.move_in_dir(dir);

        if new_position.x < 0 || new_position.y < 0 || new_position.x >= map[0].len() as i32 || new_position.y >= map.len() as i32 {
            continue;
        }

        surrounds.push(map[new_position.y as usize][new_position.x as usize]);
    }

    return surrounds;
}

fn get_basin_size(map: &Vec<Vec<u8>>, pos: Position<i32>) -> usize {
    let mut visited = HashSet::new();
    let mut to_scan = Vec::new();
    to_scan.push(pos);

    while to_scan.len() > 0 {
        let pos = to_scan.pop().unwrap();

        for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let mut new_position = pos.clone();
            new_position.move_in_dir(dir);
    
            if new_position.x < 0 || new_position.y < 0 || new_position.x >= map[0].len() as i32 || new_position.y >= map.len() as i32 {
                continue;
            }

            let x = new_position.x as usize;
            let y = new_position.y as usize;

            if map[y][x] == 9 || visited.contains(&new_position) {
                continue;
            }
    
            to_scan.push(new_position);
        }
        visited.insert(pos);
    }

    return visited.len();
}

fn main() {
    let input = must_read_input_to_lines();
    let heights: Vec<Vec<u8>> = input.iter().map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect()).collect();

    let mut sum = 0;

    let mut basins = Vec::new();
    for (y, row) in heights.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if get_adjacents(&heights, Position::new(x as i32, y as i32)).into_iter().all(|v| v > *val) {
                sum += *val as u32 + 1;

                basins.push(get_basin_size(&heights, Position::new(x as i32, y as i32)));
            }
        }
    }

    basins.sort();

    let part2 = basins[basins.len()-1] * basins[basins.len()-2] * basins[basins.len()-3];

    println!("Part1: {}", sum);
    println!("Part2: {}", part2);
}
