use std::{fmt::Debug, ops::{RangeInclusive}, str::FromStr};

const DEFAULT_DELIMITER: &str = "..";

// MaybeRange represents a value that might be a range, or it might be a static value
pub enum MaybeRange<T> {
    Static(T),
    Range(T, T)
}

impl<T> MaybeRange<T> where T: PartialOrd + FromStr + Copy + Default + std::ops::Sub<Output = T>, <T as FromStr>::Err: Debug {
    pub fn new(s: &str) -> Self {
        return MaybeRange::new_with_range_delimiter(s, DEFAULT_DELIMITER)
    }

    // Constructs a new MaybeRange from a string where ranges are seperated
    // by a given delimiter. e.g. if d == "->", then ranges are a->b
    pub fn new_with_range_delimiter(s: &str, d: &str) -> Self {
        match s.parse() {
            Ok(i) => MaybeRange::Static(i),
            Err(_) => {
                let (start, end) = s.split_once(d).map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap())).unwrap();
                MaybeRange::Range(start, end)
            }
        }
    }

    // Gets the size of the MaybeRange. For static values, this is the default value (0 generally),
    // for ranges, this is (a - b).abs()
    pub fn size(&self) -> T {
        match &self {
            MaybeRange::Static(_) => T::default(),
            MaybeRange::Range(a, b) => {
                if a > b {
                    return *a - *b
                }
                else {
                    return *b - *a
                }
            }
        }
    }

    // If this is a range, return an iterable range from a to b (inclusive). If not, panic
    pub fn get_range(&self) -> RangeInclusive<T> {
        match &self {
            MaybeRange::Static(_) => panic!(),
            MaybeRange::Range(a, b) => *a..=*b
        }
    }

    // If this is a static, return it. If not, panic
    pub fn get_static(&self) -> T {
        match &self {
            MaybeRange::Static(a) => *a,
            MaybeRange::Range(_, _) => panic!()
        }
    }
}