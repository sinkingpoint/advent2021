use std::{convert::TryInto, collections::HashMap};

use libadvent::*;

fn apply(base: Vec<char>, memoize: &mut HashMap<String, String>, conversions: &HashMap<[char;2], char>) -> Vec<char> {
    let mut new = Vec::new();

    for i in 0..base.len()-1 {
        new.push(base[i]);
        let test = &[base[i], base[i+1]];
        if let Some(c) = conversions.get(test) {
            new.push(*c)
        }
    }

    new.push(base[base.len() - 1]);

    return new;
}

fn main() {
    let input = must_read_input_to_lines();

    let mut base = Vec::new();
    for c in input[0].chars() {
        base.push(c);
    }

    let splits: HashMap<_, _> = input[2..].iter().map(|s| {
        let (a, b) = s.split_once(" -> ").unwrap();
        (a.chars().collect::<Vec<char>>().try_into().unwrap(), b.chars().next().unwrap())
    }).collect();

    let mut part1 = base.clone();
    for i in 0..10 {
        part1 = apply(part1, &splits);
        println!("After {} - {} len", i+1, part1.len());
    }

    let occurences = count_occurences(base.iter());
    let max = occurences.iter().max_by_key(|(_, c)| *c).unwrap();
    let min = occurences.iter().min_by_key(|(_, c)| *c).unwrap();

    println!("Max = {} with {}, Min = {} with {}. Result: {}", max.0, max.1, min.0, min.1, max.1 - min.1);

    let mut part2 = base.clone();
    for i in 0..40 {
        part2 = apply(part2, &splits);
        println!("After {} - {} len", i+1, part2.len());
    }
}
