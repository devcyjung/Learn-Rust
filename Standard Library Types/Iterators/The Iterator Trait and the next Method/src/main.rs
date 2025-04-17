fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let mut v1_into_iter = v1.into_iter();

    assert_eq!(v1_into_iter.next(), Some(1));
    assert_eq!(v1_into_iter.next(), Some(2));
    assert_eq!(v1_into_iter.next(), Some(3));
    assert_eq!(v1_into_iter.next(), None);

    let mut v2 = vec![1, 2, 3];
    let mut v2_iter_mut = v2.iter_mut();

    assert_eq!(v2_iter_mut.next(), Some(&mut 1));
    assert_eq!(v2_iter_mut.next(), Some(&mut 2));
    if let Some(m) = v2_iter_mut.next() {
        *m = 42;
    }
    assert_eq!(v2_iter_mut.next(), None);

    assert_eq!(v2, vec![1, 2, 42]);
}
