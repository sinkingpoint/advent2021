use std::collections::HashMap;
use itertools::Itertools;

use libadvent::*;

fn descend_with_mapping(mut working_set: Vec<&str>, actual_mapping: HashMap<char, char>) -> Option<HashMap<char, char>> {
    let segment_map: Vec<Vec<char>> = vec![
        vec!['a', 'b', 'c', 'e', 'f', 'g'],
        vec!['c', 'f'],
        vec!['a', 'c', 'd', 'e', 'g'],
        vec!['a', 'c', 'd', 'f', 'g'],
        vec!['b', 'c', 'd', 'f'],
        vec!['a', 'b', 'd', 'f', 'g'],
        vec!['a', 'b', 'd', 'e', 'f', 'g'],
        vec!['a', 'c', 'f'],
        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        vec!['a', 'b', 'c', 'd', 'f', 'g'],
    ];

    if working_set.len() == 0 {
        return Some(actual_mapping);
    }

    let (i, &value) = working_set.iter().enumerate().min_by_key(|(_, &n)| n.chars().filter(|c| !actual_mapping.contains_key(c)).count()).unwrap();
    working_set.remove(i);

    let need_to_assign: Vec<char> = value.chars().filter(|c| !actual_mapping.contains_key(c)).collect();

    if need_to_assign.len() == 0 {
        let mut translated: Vec<char> = value.chars().map(|c| actual_mapping.get(&c).unwrap()).map(|c| *c).collect();
        translated.sort();

        if !segment_map.contains(&translated) {
            return None;
        }

        return descend_with_mapping(working_set, actual_mapping);
    }
    
    let potential_numbers: Vec<&Vec<char>> = segment_map.iter().filter(|&s| {
        s.len() == value.len() && s.iter().filter(|c| !actual_mapping.values().any(|t| t == *c)).count() == need_to_assign.len()
    }).collect();

    if potential_numbers.len() == 0 {
        return None;
    }

    for number_chars in potential_numbers {
        let can_assign_to: Vec<&char> = number_chars.iter().filter(|&c|  !actual_mapping.values().any(|t| t == c)).collect();
        for perm in need_to_assign.iter().permutations(need_to_assign.len()) {
            let mut possible_assignment = actual_mapping.clone();
            for (c1, &c2) in perm.into_iter().zip(can_assign_to.iter()) {
                possible_assignment.insert(*c1, *c2);
            }

            let mut translated: Vec<char> = value.chars().map(|c| possible_assignment.get(&c).unwrap()).map(|c| *c).collect();
            translated.sort();
    
            if !segment_map.contains(&translated) {
                continue;
            }
            
            if let Some(a) = descend_with_mapping(working_set.clone(), possible_assignment) {
                return Some(a);
            }
        }
    }

    return None;
}

fn main() {
    let segment_map: Vec<Vec<char>> = vec![
        vec!['a', 'b', 'c', 'e', 'f', 'g'],
        vec!['c', 'f'],
        vec!['a', 'c', 'd', 'e', 'g'],
        vec!['a', 'c', 'd', 'f', 'g'],
        vec!['b', 'c', 'd', 'f'],
        vec!['a', 'b', 'd', 'f', 'g'],
        vec!['a', 'b', 'd', 'e', 'f', 'g'],
        vec!['a', 'c', 'f'],
        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        vec!['a', 'b', 'c', 'd', 'f', 'g'],
    ];

    let input = must_read_input_to_lines();
    let mut uniq_count = 0;
    let mut sum: i32 = 0;
    for line in input {
        let (part1, part2) = line.split_once(" | ").unwrap();

        for word in part2.split_ascii_whitespace() {
            if [segment_map[1].len(), segment_map[4].len(), segment_map[7].len(), segment_map[8].len()].contains(&word.len()) {
                uniq_count += 1;
            }
        }

        let working_set = part1.split_ascii_whitespace().collect();
        let actual_mapping = descend_with_mapping(working_set, HashMap::new()).unwrap();

        let translated = part2.split_ascii_whitespace().map(|word| {
            let mut translated_chars: Vec<char> = word.chars().map(|c| *actual_mapping.get(&c).unwrap()).collect();
            translated_chars.sort();

            println!("{} {:?}", word, translated_chars);
            let number = segment_map.iter().enumerate().filter(|(_, segment)| *segment == &translated_chars).next().unwrap();
            
            format!("{}", number.0)
        }).join("");

        sum += translated.parse::<i32>().unwrap();

        println!("Translated: {}", translated);
    }

    println!("{}", uniq_count);
    println!("{}", sum);
}
