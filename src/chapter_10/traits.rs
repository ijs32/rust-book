use std::{fmt::Display, iter::Sum};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read More...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {} // for a default implementation
                                // impl Summary for NewsArticle {
                                //     fn summarize(&self) -> String {
                                //         format!("{} by {} ({})", self.headline, self.author, self.location)
                                //     }
                                // }

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

// this works but is actually syntactic sugar for a trait bound
pub fn notify(item: impl Summary, item2: impl Summary) {
    // we can pass in tweet or news article since they both implement the Summary trait
    println!("New Article available {}", item.summarize()); // we can call any methods on item that summary implements
}
// trait bound
pub fn longer_notify<T: Summary>(item1: T, item2: T) {
    // trait bounds are stricter
    println!("New Article available {}", item1.summarize()); // same as above
}

// trait bound syntax for the below would be
// pub fn example<T: Summary + Display>(item: T) {
pub fn example(item: impl Summary + Display) {
    // this allows us to define multiple traits
    println!("New Article available {}", item.summarize());
}

// we can also create functions that return types that implement the Summary trait --
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("isilber.dev"),
        content: String::from("hahahha funny tweet"), // this only works if youre returning a single type
        reply: true,    // you couldnt have an if else statement returning either
        retweet: false, // Tweet or NewsArticle
    }
}
pub fn main() {
    let article = NewsArticle {
        headline: String::from("HEADLINE"),
        location: String::from("SC, USA"),
        author: String::from("Ian Silber"),
        content: String::from("not really important is it"),
    };

    let tweet = Tweet {
        username: String::from("isilber.dev"),
        content: String::from("hahahha funny tweet"),
        reply: true,
        retweet: false,
    };

    // let summary = notify(article, tweet); // if we tried to call longer_notify with these parameters it would not work,
    // trait bounds only allow one type of a struct
    println!("New Article available {}", article.summarize());
}
