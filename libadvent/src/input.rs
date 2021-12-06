use std::{convert::TryInto, fs::File, io::Read, str::FromStr};
use std::fmt::Debug;

// must_read_file reads the file of the given name, and panics if
// it can't for any reason
pub fn must_read_file(name: &str) -> String {
    let mut buffer = String::new();
    File::open(name).unwrap().read_to_string(&mut buffer).unwrap();

    if buffer.ends_with("\n") {
        buffer.pop();
    }

    return buffer;
}

// must_read_input reads input.txt and panics if it can't for any reason
pub fn must_read_input() -> String {
    return must_read_file("input.txt");
}

// reads input.txt, and returns a Vec of each line in the input
pub fn must_read_input_to_lines() -> Vec<String> {
    let input = must_read_input();
    return input.split("\n").map(|s| s.to_owned()).collect();
}

// reads input.txt and parses every line to an int
pub fn must_parse_input_to_ints() -> Vec<i64> {
    return must_parse_to(must_read_input_to_lines());
}

// reads input.txt and parses every line to an float
pub fn must_parse_input_to_floats() -> Vec<f64> {
    return must_parse_to(must_read_input_to_lines());
}

// calls .parse on every line, panicing if it fails
pub fn must_parse_to<T>(lines: Vec<String>) -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug {
    return lines.into_iter().map(|s| s.parse().unwrap()).collect();
}

// Trys to parse every line to an i64 and panics if that fails
pub fn must_parse_to_ints(lines: Vec<String>) -> Vec<i64> {
    return must_parse_to(lines);
}

// Trys to parse every line to an f64 and panics if that fails
pub fn must_parse_to_floats(lines: Vec<String>) -> Vec<f64> {
    return must_parse_to(lines);
}

pub fn must_split_lines_by<'a, const T: usize>(s: &'a str, sep: &str) -> Vec<[&'a str;T]> {
    let mut buf = Vec::new();
    for line in s.split("\n") {
        if line.trim().len() == 0 {
            continue
        }
        let parts = line.split(sep).map(|s| s.trim()).collect::<Vec<&str>>().try_into().unwrap();
        buf.push(parts);
    }

    return buf;
}

pub fn must_split_lines_to_csv<'a, const T: usize>(s: &'a str) -> Vec<[&'a str;T]> {
    return must_split_lines_by(s, ",")
}

pub fn must_split_into_map<T: From<char>>(s: &str) -> Vec<Vec<T>> {
    return s.split("\n").filter(|s| s.trim().len() > 0).map(|line| {
        line.chars().map(|c| c.into()).collect()
    }).collect();
}
