/*
    title: 一般的なプログラミングの概念
    subtitle: 変数と可変性、データ型
    url  : https://doc.rust-jp.rs/book/second-edition/ch03-00-common-programming-concepts.html
    name : Surpris
    date : 2020/01/29
*/

fn main() {
    /* 変数と可変性 */
    // let x = 5; // 不変変数の定義。Rust ではデフォルトで不変になる
    // println!("The value of x: {}", x);
    // x = 6; // １．不変変数なので値を変えられない
    // println!("The value of x: {}", x);

    let mut x = 5; // 可変変数の定義。Rust ではデフォルトで不変になる
    println!("The value of x: {}", x);
    x = 6; // ２．不変変数なので値を変えられる
    println!("The value of x: {}", x);

    const MAX_POINTS: u32 = 100_000; // ３．定数は const
    println!("The value of constant: {}", MAX_POINTS);

    let x = 5;
    let x = x + 1; // ４．変数は何度も宣言しなおして使用できる（Shadowing）
    let x = x * 2;
    println!("The value of x: {}", x);

    let spaces = "    ";
    let spaces = spaces.len(); // ５．同一名で別の型の値を入れられる
    println!("length of spaces: {}", spaces);

    // let mut spaces = "    "; // ６－１．mut の場合は最初の宣言で型が決まる
    // spaces = spaces.len(); // ６－２．なのでこれはコンパイルエラーとなる

    /* データ型 */
    let guess: u32 = "42".parse().expect("Not a number!"); // ７．型注釈をつけないとエラー
    println!("guess: {}", guess);

    // let x = 2.0 ; //８．デフォルトでは f64
    // let y: f32 = 3.0;

    // ９．一般的な数学演算
    // let sum = 5 + 10;
    // let diff = 95.5 - 4.3;
    // let prod = 4 * 30;
    // let quot = 56.7 / 32.2;
    // let remain = 43 % 5;

    // １０．論理値
    // let t = true;
    // let f: bool = false;

    // １１．char型。''で囲う。Unicode のスカラー値を表すので、絵文字でもいける。
    let c = 'z';
    println!("char c: {}", c);

    // １２－１．タプル型。変数はタプル全体に束縛される。
    let tup: (i32, f64, u8) = (5500, 6.4, 1);

    let (x, y, z) = tup; // １２－２．分配
    println!("values in tup: {}, {}, {}", x, y, z);
    
    let x = (400, 3, 1.3);
    println!("{}, {}, {}", x.0, x.1, x.2); // １２－３．タプルのインデックス


    // １３．配列型。ベクター型に比べると柔軟ではない
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let third = a[2];
    println!("first, third: {}, {}", first, third);
}