use std::collections::HashMap;
use std::hash::Hash;

pub fn count_occurences<T, I: IntoIterator<Item=T>>(ts: I) -> HashMap<T, usize> where T: Hash + Eq{
    let mut occurences = HashMap::new();

    for t in ts {
        let count = *occurences.get(&t).unwrap_or(&0) + 1;
        occurences.insert(t, count);
    }

    return occurences;
}

#[test]
fn test_occurences() {
    assert_eq!(*count_occurences(vec![1, 1, 2, 2, 3, 3, 3].into_iter()).get(&1).unwrap(), 2);
    assert_eq!(*count_occurences(vec![1, 1, 2, 2, 3, 3, 3].into_iter()).get(&2).unwrap(), 2);
    assert_eq!(*count_occurences(vec![1, 1, 2, 2, 3, 3, 3].into_iter()).get(&3).unwrap(), 3);
}
