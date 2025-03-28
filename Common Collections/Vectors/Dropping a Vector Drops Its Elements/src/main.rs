fn main() {
    {
        let mut v = vec![1, 2, 3, 4];
        println!("vector: {:?}", v);
        println!("vector[0] = {}", v[0]);
        println!("vector[1] = {}", v[1]);
        println!("vector[2] = {}", v[2]);
        println!("vector[3] = {}", v[3]);

        fn start<T>(x: usize, v: &[T]) -> usize {
            if v.len() > x {
                x
            } else if !v.is_empty() {
                v.len() - 1
            } else {
                0
            }
        }

        fn end<T>(x: usize, v: &[T]) -> usize {
            if v.len() >= x {
                x
            } else if !v.is_empty() {
                v.len()
            } else {
                0
            }
        }

        v.splice(start(2, &v)..end(3, &v), [30, 35]);
        println!("vector: {:?}", v);
        v.splice(start(3, &v)..end(5, &v), [350, 400]);
        println!("vector: {:?}", v);
        v.splice(start(1, &v)..end(1, &v), [5000, 5500]);
        println!("vector: {:?}", v);
        // do stuff with v
    } // <- v goes out of scope and is freed here
}
