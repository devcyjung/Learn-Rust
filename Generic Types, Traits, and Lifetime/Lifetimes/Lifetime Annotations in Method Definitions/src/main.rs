struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // The first lifetime elision rule is applied
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // The third lifetime elision rule is applied
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let imp = ImportantExcerpt { part: "Thank you" };
    println!("{}", imp.part);
    println!("{}", imp.level());
    println!(
        "{}",
        imp.announce_and_return_part("Smoking is not allowed.")
    );
}
