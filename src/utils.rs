use rand::Rng;
use std::collections::BTreeMap;
use std::collections::HashSet;

pub fn next_with_wrap<T>(i: usize, collection: &Vec<T>) -> usize {
    return (i + 1) % collection.len();
}

pub fn previous_with_wrap<T>(i: usize, collection: &Vec<T>) -> usize {
    return (i + collection.len() - 1) % collection.len();
}

pub fn roll_dice() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

pub fn has_duplicates<T: Eq + std::hash::Hash>(vec: &Vec<T>) -> bool {
    let mut set = HashSet::new();

    for item in vec {
        if !set.insert(item) {
            return true;
        }
    }

    return false;
}

pub fn has_duplicate_values<T, U>(map: &BTreeMap<T, U>) -> bool
where
    U: std::cmp::Eq + std::hash::Hash,
{
    let mut seen_values = HashSet::new();

    for value in map.values() {
        if !seen_values.insert(value) {
            return true;
        }
    }

    false
}
