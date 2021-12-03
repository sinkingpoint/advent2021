use libadvent::*;

fn main() {
    let input = must_read_input_to_lines();
    let mut gamma = 0;
    let mut epsilon = 0;
    for c in (0..input[0].len()).map(|i| count_chars_at(&input, i)) {
        if c.get(&'0') > c.get(&'1')  {
            gamma = gamma << 1 | 0;
            epsilon = epsilon << 1 | 1;
        }
        else {
            gamma = gamma << 1 | 1;
            epsilon = epsilon << 1 | 0;
        }
    }

    println!("{} x {} = {}", gamma, epsilon, gamma * epsilon);

    let mut oxygen_generator = input.clone();

    let mut i = 0;
    while oxygen_generator.len() > 1 {
        let c = count_chars_at(&oxygen_generator, i);

        let bit = if c.get(&'0') > c.get(&'1')  {
            '0'
        }
        else {
            '1'
        };

        oxygen_generator = oxygen_generator.into_iter().filter(|s| s.chars().nth(i).unwrap() == bit).collect();
        i += 1;
    }

    let mut co2_scrubber = input.clone();

    let mut i = 0;
    while co2_scrubber.len() > 1 {
        let c = count_chars_at(&co2_scrubber, i);

        let bit = if c.get(&'0') > c.get(&'1') {
            '1'
        }
        else {
            '0'
        };

        co2_scrubber = co2_scrubber.into_iter().filter(|s| s.chars().nth(i).unwrap() == bit).collect();
        i += 1;
    }

    let oxygen = parse_to_base(&oxygen_generator[0], 2);
    let co2 = parse_to_base(&co2_scrubber[0], 2);

    println!("{} x {} = {}", oxygen, co2, oxygen * co2);
}
