use std::iter::Sum;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize());
}

pub fn notify_function<T: Summary + Clone>(item: T) {
    println!("Hey {}", item.summarize());
}

pub fn notify_other_function<T>(item: T)
where
    T: Clone + Summary,
{
    println!("Hey {}", item.summarize());
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} by {}", self.content, self.username)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn page(&self) -> String {
        String::from("Read more..")
    }
}

fn main() {
    let na = NewsArticle {
        author: String::from("Mike"),
        headline: String::from("The best news here"),
        content: String::from("The best content "),
    };
    let t = Tweet {
        username: String::from("hey"),
        content: String::from("hey"),
        reply: true,
        retweet: false,
    };
    println!("{}", na.summarize());
    println!("{}", t.summarize());
    t.page();

    notify(&na);
}
