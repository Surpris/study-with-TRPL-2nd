/*
    title: ジェネリック型、トレイト、ライフタイム
    name : Surpris
    date : 2020/02/09
*/

extern crate chap10; // 自身を呼び出す
use chap10::Summary; // トレイト内で実装されたメソッドを使う場合は、トレイト自身も呼び出す必要がある
use chap10::Test;
use chap10::Tweet; // 自身の構造体の呼び出し
use std::fmt::Display;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 7];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);

    // let char_list = vec!['y', 'r', 'a', 'q'];
    // let result = largest_2(&char_list);
    // println!("The largest char is {}", result);

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    let _i_and_f = Point { x: 5, y: 4.0 };

    println!("p.x = {0}, p.y = {1}", _i_and_f.x(), _i_and_f.y());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("{}, {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("house_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let test = Test {
        username: String::from("test"),
        user_id: String::from("0001"),
    };

    println!("New article available! {}", test.summarize());

    let char_list = vec!['y', 'r', 'a', 'q'];
    let result = largest_3(&char_list);
    println!("The largest char is {}", result);

    let s = 3.to_string();
    /*
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
    */

    let string1 = String::from("abcd");
    let string2 = "acb";

    let result = longest(string1.as_str(), string2);
    println!("The longer string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'."); // .next() は iterator 内の現在位置の要素（の参照）を返して次の要素に（内部的に）移動する。
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

fn largest(list: &[i32]) -> i32 {
    // 所有権を移さないように参照
    let mut largest = list[0];

    for &item in list.iter() {
        // iter() が返す値を参照する
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest_2<T>(list: &[T]) -> T {
//     // １．普通に T を与えるだけでは T に比較演算子が定義されていないのでエラーが出る
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if largest < item {
//             largest = item;
//         }
//     }
//     largest
// }

fn largest_3<T: PartialOrd + Copy>(list: &[T]) -> T {
    // １１．PartialOrd トレイトを実装している T のみ使用可能にする
    // １２．Copy トレイトを実装している T のみ使用可能にする
    let mut largest = list[0];

    for &item in list.iter() {
        if largest < item {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    // ２．構造体と列挙型の場合はジェネリック型を宣言するだけでよい
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // ３．メソッドの定義ではジェネリック型を宣言する必要がある
    fn x(&self) -> &T {
        // ４．メンバーと同名の変数を定義することができる
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        // ６．構造体定義とはことなるジェネリックな型を使用するメソッド
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    // ５．特定の型の構造体にのみ使えるメソッドの実装
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // １３．シグニチャの全参照が同じライフタイムになると指定
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
    // １４．最初の引数をそのまま返す場合は、第二の引数にライフタイムを与える必要がない
    x
}

/*
fn longest_3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() // １５．関数内にある変数の参照を返そうとするとエラー（ダングリング参照を与えるため）
}
*/

struct ImportantExcerpt<'a> {
    // １６．構造体へのライフタイム注釈
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // １７－１．ライフタイム注釈を与えるメソッドの実装
    fn level(&self) -> i32 {
        // １７－２．参照を返さない関数も定義可能
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // １７－３．&self と announcement に別々のライフタイムが与えられ、戻り値には &self と同じライフタイムが与えられる
        println!("Attention please: {}", announcement);
        self.part // １７－４．&self と同じライフタイムである必要がある
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    // １８．ジェネリックとトレイト境界とライフタイムが含まれる関数の例
    // T に対するトレイト境界を where キーワードによって与えている
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
