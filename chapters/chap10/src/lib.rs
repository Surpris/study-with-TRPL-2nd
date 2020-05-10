/*
    title: ジェネリック型、トレイト、ライフタイム
    subtitle: トレイト
    name : Surpris
    date : 2020/02/17
*/

use std::fmt::Display;

pub trait Summary {
    // ７．トレイトの定義
    fn summarize(&self) -> String {
        // ９．デフォルト実装
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // ８－１．トレイト内のメソッドを NewsArticle 構造体に実装する
    fn summarize(&self) -> String {
        // ８－２．実装の詳細
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
    // ８－３．別の構造体に対する実装
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Test {
    pub username: String,
    pub user_id: String,
}

impl Summary for Test {}

pub fn notify<T: Summary>(item: T) {
    // １０．Summary トレイトを実装していないと使用できない関数の実装
    println!("Breaking news! {}", item.summarize());
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // １３－１．こちらはすべての T を持つ Pair に対して実装される
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // １３－２．こちらは Display と PartialOrd を実装する T を持つ Pair に対して実装される
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The larger number is x = {}", self.x);
        } else {
            println!("The large number is y = {}", self.y);
        }
    }
}
