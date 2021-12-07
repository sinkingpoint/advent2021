use libadvent::*;

fn main() {
    let input = must_read_input();

    // Part 1 we can cheese with the median
    let mut numbers: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
    numbers.sort();

    let median = numbers[numbers.len()/2];
    let steps: i64 = numbers.iter().map(|i| (i - median).abs()).sum();
    println!("Steps: {}", steps);

    // Part 2 is very naive, but with the range of the input it seems fine
    let max: i64 = *numbers.iter().max().unwrap();
    let min: i64 = (1..max).map(|pos| numbers.iter().map(|i| {
        // Calculate the cost of this move with 1+2+3...n = (n)(n+1)/2
        let steps = (i - pos).abs();
        steps * (steps + 1) / 2
    }).sum()).min().unwrap();

    println!("Part 2: {}", min);
}
