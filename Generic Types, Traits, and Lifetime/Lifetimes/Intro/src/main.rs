fn main() {
    let v;
    let s;
    {
        let v1: &'static [i32] = &[1, 2, 3];
        v = v1;
        let s2: &'static str = "Static";
        s = s2;
    }
    println!("{:?}\n{}", v, s);
}
