pub trait Summary {
    fn summarize(&self) -> String;
    fn babble(&self) {
        println!("I got babble!");
        println!("{}", self.summarize());
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
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
    fn babble(&self) {
        println!("Overriden babble for Tweets");
        println!("{}", self.summarize());
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let news = NewsArticle {
        headline: String::from("Headline of the news"),
        author: String::from("Author of the news"),
        location: String::from("Location of the news"),
        content: String::from("Contents of the news"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new news summary: {}", news.summarize());

    tweet.babble();
    news.babble();
}
