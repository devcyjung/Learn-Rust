#[must_use]
pub fn vec_map(v: &[i32]) -> Vec<i32> {
    v.iter().map(|num| 2 * num).collect()
}
