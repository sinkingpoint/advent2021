use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn step1(mut counts: [i64; 9], steps: usize) -> [i64; 9]{
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

pub fn step2(mut counts: [i64; 9], steps: usize) -> [i64; 9]{
    let len = counts.len();
    for i in 0..steps {
        counts[(i + 7) % len] += counts[i % len];
    }

    return counts;
}

fn criterion_benchmark(c: &mut Criterion) {
    let counts = [0, 1, 1, 2, 1, 0, 0, 0, 0];
    assert_eq!(step1(counts, 80).iter().sum::<i64>(), 5934);
    assert_eq!(step1(counts, 256).iter().sum::<i64>(), 26984457539);
    assert_eq!(step2(counts, 80).iter().sum::<i64>(), 5934);
    assert_eq!(step2(counts, 256).iter().sum::<i64>(), 26984457539);
    
    c.bench_function("step1 80", |b| b.iter(|| step1(black_box(counts), black_box(80))));
    c.bench_function("step1 256", |b| b.iter(|| step1(black_box(counts), black_box(256))));

    c.bench_function("step2 80", |b| b.iter(|| step2(black_box(counts), black_box(80))));
    c.bench_function("step2 256", |b| b.iter(|| step2(black_box(counts), black_box(256))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);