pub fn create_array() -> [i32; 4] {
    [10, 20, 30, 40]
}

pub fn create_vector() -> Vec<i32> {
    Vec::from(create_array())
}
