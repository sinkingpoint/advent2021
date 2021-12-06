use libadvent::*;

fn step(mut counts: [i64; 9], steps: usize) -> [i64; 9]{
    for _ in 0..steps {
        let new_babies = counts[0];
        for j in 1..9 {
            counts[j-1] = counts[j];
        }

        counts[8] = new_babies;
        counts[6] += new_babies;
    }

    return counts;
}

fn main() {
    let input = must_read_input();

    let fish: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let mut counts = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in fish {
        counts[i] += 1;
    }

    println!("Fish after 80 days: {}", step(counts, 80).iter().sum::<i64>());
    println!("Fish after 256 days: {}", step(counts, 256).iter().sum::<i64>());
}

#[test]
fn test() {
    let counts = [0, 1, 1, 2, 1, 0, 0, 0, 0];
    assert_eq!(step(counts, 80).iter().sum::<i64>(), 5934);
    assert_eq!(step(counts, 256).iter().sum::<i64>(), 26984457539);
}
