use std::{iter::Step, ops::{MulAssign, Rem, Sub, SubAssign}};

// Given a list of (n, c) where x % c == n, find x
pub fn chinese_remainder_theorum<V, T: Iterator<Item=(V, V)>>(mut iter: T) -> V where V: Copy + Into<usize> + Step + MulAssign + Sub<Output = V> + Rem<Output = V> {
    let (a1, n1) = iter.next().unwrap();
    let mut congruence =  n1 - a1 % n1;
    let mut step = n1;

    for (ai, ni) in iter {
        congruence = (congruence..).step_by(step.into()).filter(|x| *x % ni == ni - ai % ni).next().unwrap();
        step *= ni;
    }

    return congruence;
}

// Given (a, b) find max(c) where a % c == 0 && b % c == 0
pub fn gcd<V>(mut a: V, mut b: V) -> V where V: Copy + PartialEq + PartialOrd + SubAssign {
    while a != b {
        if a > b {
            a -= b;
        }
        else {
            b -= a;
        }
    }

    return a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(3, 9), 3);
    assert_eq!(gcd(8, 9), 1);
    assert_eq!(gcd(64, 16), 16);
    assert_eq!(gcd(12, 16), 4);
}

pub fn parse_to_base(s: &str, base: u64) -> u64 {
    let mut r = 0;
    for (i, c) in s.chars().rev().enumerate() {
        let val = match c {
            '0'..='9' => (c as u64 - '0' as u64),
            'A'..='Z' => (c as u64 - 'A' as u64) + 10,
            'a'..='z' => (c as u64 - 'a' as u64) + 10,
            _ => panic!()
        };

        r += val * base.pow(i as u32);
    }

    return r;
}

#[test]
fn test_parse_to_base() {
    assert_eq!(parse_to_base("00000", 2), 0b00000);
    assert_eq!(parse_to_base("00001", 2), 0b00001);
    assert_eq!(parse_to_base("10000", 2), 0b10000);
    assert_eq!(parse_to_base("FF", 16), 0xFF);
}