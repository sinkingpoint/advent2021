use libadvent::*;

#[derive(Debug, Clone)]
enum SnailFishNumber {
    Literal(u32),
    Pair(Box<SnailFishNumber>, Box<SnailFishNumber>)
}

fn parse_number(input_chars: &[char]) -> (usize, SnailFishNumber) {
    if input_chars[0] == '[' {
        let mut base = 1;
        let (idx, left) = parse_number(&input_chars[1..]);
        assert_eq!(input_chars[idx+1], ',');
        base += idx+1;// +1 to skip the ,
        let (idx, right) = parse_number(&input_chars[base..]);
        base += idx;

        assert!(input_chars[base] == ']');

        return (base + 1, SnailFishNumber::Pair(Box::new(left), Box::new(right)));
    }
    else {
        let last_char_pos = input_chars.iter().position(|c| !c.is_ascii_digit()).unwrap_or(input_chars.len());
        let s = {
            let mut s = String::with_capacity(last_char_pos);
            for c in &input_chars[..last_char_pos] {
                s.push(*c);
            }

            s
        };

        return (last_char_pos, SnailFishNumber::Literal(s.parse().unwrap()));
    }
}

fn flatten(s: &SnailFishNumber) -> Vec<(u32, u32)> {
    match s {
        SnailFishNumber::Literal(i) => vec![(0, *i)],
        SnailFishNumber::Pair(a, b) => {
            let mut out = Vec::new();
            for (depth, s) in flatten(a) {
                out.push((depth+1, s));
            }

            for (depth, s) in flatten(b) {
                out.push((depth+1, s));
            }

            out
        }
    }
}

fn explode_step(mut flattened: Vec<(u32, u32)>) -> (Vec<(u32, u32)>, bool) {
    let mut out: Vec<(u32, u32)> = Vec::new();
    let mut i = 0;
    let mut exploded = false;
    while i < flattened.len() {
        if flattened[i].0 == 5 && !exploded {
            // We need to explode!
            if i > 0 {
                let len = out.len()-1;
                out[len].1 += flattened[i].1;
            }

            assert!(flattened[i+1].0 == 5);

            if i < flattened.len() - 2 {
                flattened[i+2].1 += flattened[i+1].1;
            }

            out.push((flattened[i].0-1, 0));
            i += 1;
            exploded = true;
        }
        else {
            out.push(flattened[i]);
        }

        i += 1;
    }
    return (out, exploded);
}

fn split_step(flattened: Vec<(u32, u32)>) -> (Vec<(u32, u32)>, bool) {
    let mut out: Vec<(u32, u32)> = Vec::new();
    let mut i = 0;
    let mut split = false;
    while i < flattened.len() {
        if flattened[i].1 >= 10 && !split {
            let left = flattened[i].1 / 2;
            let right = (flattened[i].1 + 1) / 2;

            out.push((flattened[i].0+1, left));
            out.push((flattened[i].0+1, right));
            split = true;
        }
        else {
            out.push(flattened[i]);
        }

        i += 1;
    }
    return (out, split);
}

fn reduce(mut flattened: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    loop {
        let mut did_action = false;
        loop {
            let (f, ex) = explode_step(flattened);
            did_action |= ex;
            flattened = f;

            if !ex {
                break;
            }
        }

        let (f, ex) = split_step(flattened);
        did_action |= ex;
        flattened = f;

        if !did_action {
            break;
        }
    }

    return flattened;
}

fn step_magnitude(flatten: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let max_depth = flatten.iter().max_by_key(|f| f.0).unwrap().0;
    let mut out = Vec::new();
    let mut i = 0;
    while i < flatten.len() {
        if flatten[i].0 == max_depth {
            out.push((flatten[i].0-1, 3 * flatten[i].1 + flatten[i+1].1 * 2));
            i += 1;
        }
        else {
            out.push(flatten[i]);
        }

        i += 1;
    }

    out
}

fn magnitude(mut flatten: Vec<(u32, u32)>) -> u32 {
    let mut max_depth = flatten.iter().max_by_key(|f| f.0).unwrap().0;
    while max_depth > 1 {
        flatten = step_magnitude(flatten);
        max_depth = flatten.iter().max_by_key(|f| f.0).unwrap().0;
    }

    assert_eq!(flatten.len(), 2);
    return flatten[0].1 * 3 + flatten[1].1 * 2;
}

fn add(s1: Vec<(u32, u32)>, s2: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut out = Vec::new();

    for i in s1 {
        out.push((i.0+1, i.1));
    }

    for i in s2 {
        out.push((i.0+1, i.1));
    }

    out
}

fn main() {
    let input = must_read_input_to_lines();
    let mut num = flatten(&parse_number(&input[0].chars().collect::<Vec<char>>()).1);
    
    for line in &input[1..] {
        let next = flatten(&parse_number(&line.chars().collect::<Vec<char>>()).1);
        num = reduce(add(num, next));
    }

    println!("Part 1: {}", magnitude(num));

    let mut max_magnitude = 0;
    for i in 0..input.len() {
        for j in i+1..input.len() {
            let a = flatten(&parse_number(&input[i].chars().collect::<Vec<char>>()).1);
            let b = flatten(&parse_number(&input[j].chars().collect::<Vec<char>>()).1);

            let mag_a = magnitude(reduce(add(a.clone(), b.clone())));
            let mag_b = magnitude(reduce(add(b.clone(), a.clone())));

            if mag_a > max_magnitude {
                max_magnitude = mag_a;
            }

            if mag_b > max_magnitude {
                max_magnitude = mag_b;
            }
        }
    }

    println!("Max: {}", max_magnitude);
}
