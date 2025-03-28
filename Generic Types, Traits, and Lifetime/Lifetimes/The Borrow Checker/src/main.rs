fn main() {
    // Annotations of the lifetimes of `r` and `x`,
    // named `'a` and `'b`, respectively
    {
        let r: &i32; // 'a

        {
            // let x: i32 = 5; // 'b
            // r = &x; // 'b ends
            const Y: i32 = 5; // 'c
            r = &Y; // 'c doesn't end. (same for static Y)
        }

        println!("r: {}", r);
    } // 'a ends

    // A valid reference because the data has a longer
    // lifetime than the reference
    {
        let x: i32 = 5; // 'b

        let r: &i32 = &x; // 'a

        println!("r: {}", r); // 'a ends, 'b ends
    }
}
