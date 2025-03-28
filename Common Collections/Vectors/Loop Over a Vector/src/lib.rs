pub fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    v.iter_mut().for_each(|i| *i *= 2);
    v
}
