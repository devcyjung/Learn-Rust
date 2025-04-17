#[test]
fn iterator_sum() {
    let v1 = [1, 2, 3, 4, 5];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 15);

    assert_eq!((1..=10).sum::<i32>(), 55);
}
