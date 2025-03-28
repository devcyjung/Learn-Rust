fn main() {
    let s;
    let s2;
    let s3;
    {
        s = "I have a static lifetime.";
        let s2v2: &str = "I have a static lifetime.";
        s2 = s2v2;
        let s3v2: &'static str = "Static";
        s3 = s3v2;
    }
    println!("{}", s);
    println!("{}", s2);
    println!("{}", s3);
}
