fn main() {
    println!("Traits in Rust");

    let tweet = Tweet {
        username: String::from("Sudarshan"),
        content: String::from("Content 1"),
        is_reply: false,
        is_retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Headline 1"),
        location: String::from("Bengaluru"),
        author: String::from("Sudarshan"),
        content: String::from("News article content"),
    };

    println!("1 new tweet ->  {}", tweet.summarize());
    println!("1 new article -> {}", article.summarize());
    println!("{:?}", notify(&article));
}

pub trait Summary {
    // abstract way
    //fn summarize(&self) -> String;

    // default behaviour defined by the method
    fn summarize(&self) -> String {
        String::from("Read More...")
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, with {}, by {} from {}", self.headline, self.content, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub is_reply: bool,
    pub is_retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}. Reply ? {}, Retweet ? {}", self.username, self.content, self.is_reply, self.is_retweet)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news -> {}", item.summarize());
}