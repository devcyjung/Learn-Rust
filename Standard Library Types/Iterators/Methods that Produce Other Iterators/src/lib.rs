#[test]
fn use_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2 = v1.iter().map(|x| x + 1).collect::<Vec<_>>();

    assert_eq!(v2, vec![2, 3, 4]);
}
