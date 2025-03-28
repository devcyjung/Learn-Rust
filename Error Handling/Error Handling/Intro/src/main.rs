use tap::tap::Tap;
fn main() {
    // put your code here to launch it
    let v = [2, 3, 1, 4, 5].tap_mut(|x| x.sort());
    for i in 0.. {
        if let Some(e) = v.get(i) {
            println!("{} : {}", i, e);
        } else {
            break;
        }
    }
}
