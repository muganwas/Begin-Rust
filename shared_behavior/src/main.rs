mod aggregator;

use std::fmt::Display;

use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let la_tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let str = "Hello, world!";
    let str2 = String::from("Hello, world!");
    _summarize_1(&la_tweet);
    _summarize(&str);
    _notify(str2);
    let article = _new_article(String::from("This article is wild"));
    println!("Article: {}", article.summarize());
}

fn _summarize(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

fn _summarize_1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn _new_article(content: impl Summary + Display) -> impl Summary {
    NewsArticle {
        headline: String::from("horse ebooks 1"),
        content: content.summarize(),
        location: String::from("Los Angeles, CA"),
        author: String::from("horse_ebooks"),
    }
}

fn _summarize_2<T, U>(item: &T, item2: &U)
where
    T: Display + Summary,
    U: Summary,
{
    println!("Breaking news! {}", item.summarize());
    println!("More news! {}", item2.summarize());
}

fn _notify<T: Summary + Display>(item: T) {
    println!("Summary! {}", item.summarize());
}
