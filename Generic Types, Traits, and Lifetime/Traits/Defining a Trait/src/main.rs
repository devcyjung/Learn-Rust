pub trait Summary {
    fn summarize(&self) -> String;

    fn babble(&self) {
        println!("This is the summary. {}", self.summarize());
    }
}

fn main() {
    let tweet_content = &mut ['\0'; 280];
    for (idx, char) in "Nice tweet!".chars().enumerate() {
        if idx >= tweet_content.len() {
            println!("Out of bounds!");
            break;
        }
        tweet_content[idx] = char;
    }
    let tweet = Tweet {
        username: String::from("Alex"),
        content: tweet_content,
    };
    let news = NewsArticle {
        headline: String::from("No one died today"),
        author: String::from("Kim Jones"),
        content: String::from("Surprisingly, this was true."),
        location: (24, 157),
    };
    tweet.babble();
    news.babble();
}

struct Tweet<'a> {
    username: String,
    content: &'a [char; 280],
}

impl<'a> Summary for Tweet<'a> {
    fn summarize(&self) -> String {
        let mut content_str = String::new();
        for &c in self.content {
            content_str.push(c);
        }

        format!("User {} said: {}", self.username, content_str)
    }
}

struct NewsArticle {
    headline: String,
    author: String,
    content: String,
    location: (u32, u32),
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "Title: {}, by {}, at ({},{})\nContent: {}",
            self.headline, self.author, self.location.0, self.location.1, self.content
        )
    }
}
