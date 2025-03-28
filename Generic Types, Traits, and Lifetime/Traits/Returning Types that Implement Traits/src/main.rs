use std::fmt;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}: {}", self.headline, self.author, self.content)
    }
}

// Returning Types that Implement Traits
fn returns_a_summary_impl() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn returns_many_summary_impls(is_tweet: bool) -> Box<dyn Summary> {
    match is_tweet {
        true => Box::new(returns_a_summary_impl()),
        false => Box::new(NewsArticle {
            headline: String::from("Paint dried"),
            author: String::from("Reporter"),
            content: String::from("It really did"),
        }),
    }
}

impl fmt::Display for Box<dyn Summary> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summarize())
    }
}

fn main() {
    for b in [true, false] {
        let summary = returns_many_summary_impls(b);
        println!("{}\n{}", summary.summarize(), summary);
    }
}
