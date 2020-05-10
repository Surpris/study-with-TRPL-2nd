/*
    title: 一般的なプログラミングの概念
    subtitle: フロー制御
    url  : https://doc.rust-jp.rs/book/second-edition/ch03-00-common-programming-concepts.html
    name : T. N.
    date : 2020/01/29
*/

fn main() {
    let num = 3;
    
    if num < 5 { // １．if文
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if num % 4 == 0 {
        println!("number is divisible by 4.");
    } else if num % 3 == 0 { // ２．else if 文
        println!("number is divisible by 3.");
    } else if num % 2 == 0 {
        println!("number is divisible by 2.");
    } else {
        println!("number is not divisible by 4, 3, or 2.");
    }

    let condition = true;
    let num = if condition { // ３．match と同じく式なので値を返して let で束縛できる
        5
    } else {
        // "six" // ４．if 文の各アームの結果になる可能性がある値の型はすべて同じでなくてはならない。
        6
    };

    println!("The value of num is {}.", num);
}
