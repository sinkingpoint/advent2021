use std::{convert::TryInto, collections::VecDeque};

use libadvent::*;

const MAP_SIZE: usize = 10;
const MAP_ISIZE: isize = MAP_SIZE as isize;

type Map<T> = [T;MAP_SIZE*MAP_SIZE];

fn step(map: &mut Map<u8>) -> u32 {
    let mut to_process = VecDeque::new();
    for p in 0..map.len() {
        map[p] += 1;
        if map[p] > 9 {
            to_process.push_front(p);
        }
    }

    let mut flashed: Map<bool> = [false; MAP_SIZE * MAP_SIZE];

    while to_process.len() > 0 {
        let p = to_process.pop_front().unwrap();
        if flashed[p] {
            continue;
        }
        flashed[p] = true;

        let p = p as isize;
        let current_row = p / MAP_ISIZE;
        let current_column = p % MAP_ISIZE;
        
        for y in current_row-1..=current_row+1 {
            for x in current_column-1..=current_column+1 {
                if x >= 0 && x < MAP_ISIZE && y >= 0 && y < MAP_ISIZE && !(x == current_column && y == current_row)  {
                    let p2 = (y * MAP_ISIZE + x) as usize;
                    map[p2] += 1;
                    if map[p2] > 9 {
                        to_process.push_front(p2);
                    }
                }
            }
        }
    }

    let mut changed = 0;
    for i in 0..map.len() {
        if flashed[i] {
            changed += 1;
            map[i] = 0;
        }
    }

    return changed;
}

fn main() {
    let input = must_read_input_to_lines();
    let mut map: Map<u8> = input.into_iter().map(|s| s.chars().collect::<Vec<char>>()).flatten().map(|x| x as u8 - '0' as u8).collect::<Vec<u8>>().as_slice().try_into().unwrap();

    let mut num = 0;
    let mut i = 0;
    loop {
        let num_flashes = step(&mut map);
        if num_flashes == (MAP_SIZE * MAP_SIZE) as u32 {
            break
        }

        if i == 100 {
            println!("Part1: {} flashes after 100 steps", num);
        }
        num += num_flashes;
        i += 1;
    }
    println!("Synced after {} steps", i+1);
}
