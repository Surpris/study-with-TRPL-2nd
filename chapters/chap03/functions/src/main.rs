/*
    title: 一般的なプログラミングの概念
    subtitle: 関数
    url  : https://doc.rust-jp.rs/book/second-edition/ch03-00-common-programming-concepts.html
    name : Surpris
    date : 2020/01/29
*/

fn main() {
    println!("Hello, world!");
    another_function();
    another_function_2(5);
    another_function_3(5, 6);

    // let x = (let y = 6); // ３．式指向言語なのでこのような書き方は許されていない

    let y = { // ４－１．スコープは式である
        let x = 3;
        x + 1 // ４－２．ここで x+1 という値を返す
    };

    println!("The value of y is {}.", y);

    let x = five();
    println!("The value of x is {}.", x);

    let x = plus_one(5);
    println!("The value of x is {}.", x);
}

fn another_function(){ // １．関数の宣言
    println!("Another function.");
}

fn another_function_2(x: i32){ // ２．引数ありの関数
    println!("The value of x is {}.", x);
}

fn another_function_3(x: i32, y: i32){ // ２．引数ありの関数
    println!("The values of x and y are {} and {}.", x, y);
}

fn five() -> i32 { // ５．戻り値のある関数
    5
}

fn plus_one(x: i32) -> i32 {
    // x + 1; // ６．これは式ではなく文である（`;` がある）ので、最後に書くとエラー
    x + 1
}