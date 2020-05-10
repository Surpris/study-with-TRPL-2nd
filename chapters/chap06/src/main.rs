/*
    title: 列挙型
    name : Surpris
    date : 2020/02/02
*/

fn main() {
    // １－２．IpAddrKind 型の列挙子はすべて IpAddrKind 型をもつ。
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // println!("{0}, {1}", four, six);

    route(four);
    route(six); // ２－２．IpAddrKind のすべての列挙子に対して適用できる

    let _home = IpAddr {
        // ３－１．
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _home = IpAddr2::V4(String::from("127.0.0.1")); // ３－２．

    let _home = IpAddr3::V4(127, 0, 0, 1);
    let _loopback = IpAddr3::V6(String::from("::1")); // ４－２．異なる型の情報を付与

    let m = Message::Write(String::from("hello"));
    m.call(); // ６－２．それぞれの列挙子でメソッドが使用できる

    let some_number = Some(5); // ７－１．Option<T> の使用
    let _some_string = Some("a string");
    let absent_number: Option<i32> = None; // ７－２．Option<T> により null を与える。この場合、型を教える必要がある

    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alabama);

    let _value_coin1 = value_in_cents(coin1);
    let _value_coin2 = value_in_cents(coin2);

    let _some_number = plus_one(some_number);
    let _absent_number = plus_one(absent_number);

    let some_u8_value = 0u8; // １２．`0` については、型を指定するのが良さそう
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    };

    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        // １３．if let 文の使用例
        println!("three");
    } else {
        println!("another");
    }
}

enum IpAddrKind {
    // １－１．enum で列挙型を定義
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) { // ２－１．IpAddrKind を引数にとる関数
}

struct IpAddr {
    // ３－１．IpAddrKind と address を関連付ける方法の一つ
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    // ３－２．各列挙子に（address の）情報を付与できる
    V4(String),
    V6(String),
}

enum IpAddr3 {
    // ４－１．各列挙子に付与する情報の型は異なっていてよい
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    // ５．様々な型の列挙子をも足せる
    Quit,                       // 一般的な列挙子
    Move { x: i32, y: i32 },    // 匿名構造体を持つ
    Write(String),              // String 型の情報を持つ
    ChangeColor(i32, i32, i32), // ３つの i32 型の情報を持つ
}

impl Message {
    // ６．enum にもメソッドを実装できる
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // １０－１．enum を情報に持たせる
}

fn value_in_cents(coin: Coin) -> u32 {
    // ８．match の使用例
    match coin {
        Coin::Penny => {
            // ９．各ケースで `{}` ブロックを定義可能
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // １０－２．enum の情報を参照してブロック内で処理が可能
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // １１．Option<i32> を引数にもち、その値を更新する関数
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
