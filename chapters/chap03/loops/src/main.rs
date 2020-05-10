/*
    title: 一般的なプログラミングの概念
    subtitle: フロー制御、ループでの繰り返し
    url  : https://doc.rust-jp.rs/book/second-edition/ch03-00-common-programming-concepts.html
    name : Surpris
    date : 2020/01/29
*/


fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut num = 3;

    while num != 0 { // １．while ループの停止
        println!("{},", num);
        num -= 1;
    }; // ループのスコープに対してセミコロンをつけてもつけなくてもよい。

    println!("Lift off!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 { // ２．配列の要素の呼び出しループ（悪い例）
        println!("the value is {}", a[index]);
        index += 1;
    };

    for element in a.iter() { // ３．良い例
        println!("the value is {}", element);
    }

    for num in (1..4).rev() { // ４．Range 型を使ったループ
        println!("{}, ", num);
    }
    println!("Lift off!");
}