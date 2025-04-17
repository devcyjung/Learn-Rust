use std::{collections::HashMap, hash::BuildHasher};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    None,
    Some,
    Complete,
}

#[must_use]
pub fn count_for<S: BuildHasher>(map: &HashMap<String, Progress, S>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

#[must_use]
pub fn count<S: BuildHasher>(map: &HashMap<String, Progress, S>, value: Progress) -> usize {
    map.iter().filter(|(_, v)| **v == value).count()
}

#[must_use]
pub fn count_stack_for<S: BuildHasher>(
    stack: &[HashMap<String, Progress, S>],
    value: Progress,
) -> usize {
    let mut count = 0;
    for map in stack {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

#[must_use]
pub fn count_stack<S: BuildHasher>(
    stack: &[HashMap<String, Progress, S>],
    value: Progress,
) -> usize {
    stack
        .iter()
        .flat_map(|m| m.values())
        .filter(|v| **v == value)
        .count()
}
