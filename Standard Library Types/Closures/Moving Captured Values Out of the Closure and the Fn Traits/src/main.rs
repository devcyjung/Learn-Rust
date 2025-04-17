fn main() {
    //          Moves,   Mutably Borrows,    Immutably Borrows,  Captures Nothing
    // FnOnce   o        o                   o                   o
    // FnMut    x        o                   o                   o
    // Fn       x        x                   o                   o
    let x = 1;
    let f1 = move || x.to_string();
    let mut y = 2;
    let f2 = || {
        y += 1;
    };
    let z = 3;
    let f3 = || z.to_string();
    fn_once(f1);
    fn_mut(f2);
    fn_(f3);
}

// Alternative notation
// fn fn_once<F>(f: F)
// where F: FnOnce() -> String {}
fn fn_once(f: impl FnOnce() -> String) {
    println!("1. fn_once {}", f());
}

fn fn_mut(mut f: impl FnMut()) {
    f();
    println!("2. fn_mut");
}

fn fn_(f: impl Fn() -> String) {
    println!("3. fn {}", f());
}
