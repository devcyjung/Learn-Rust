fn main() {
    {
        let r;

        {
            // let X = 5;
            static X: i32 = 5;
            r = &X; // !!! ERROR: `x` does not live long enough
        }

        println!("r: {}", r);
    }
}
