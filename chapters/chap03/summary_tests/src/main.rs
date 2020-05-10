/*
    title: 一般的なプログラミングの概念
    subtitle: まとめのテスト
    url  : https://doc.rust-jp.rs/book/second-edition/ch03-00-common-programming-concepts.html
    name : Surpris
    date : 2020/01/29
*/

fn main() {
    println!("Chapter 3 test!");
    println!("C to F");
    let t_c = 27.0;
    println!("Celsius: {}", t_c);
    let t_f = c_to_f(t_c);
    println!("Fahrenheit: {}", t_f);

    println!("Fibonacci number");
    const N: i32 = 10;
    let mut x_n = 1;
    let mut x_n_1 = 1;
    for x in 1..N {
        if x != 1 && x != 2 {
            let buff = x_n * 1;
            x_n = fibonacci_number(x_n, x_n_1);
            x_n_1 = buff;
        }
        println!("{}, {}", x, x_n);
    }

}

/*
    c_to_f(temp_c: f64) -> f64
    摂氏温度を華氏温度に変換する
    式は　F = C * 9 / 5 + 32
    <param>temp_c</param>
    <return>華氏温度</return>
*/
fn c_to_f(temp_c: f64) -> f64 {
    temp_c * 9.0 / 5.0 + 32.0
}

/*
    fibonacci_number(x_n: i32, x_n_1: i32) -> i32
    フィボナッチ数列を与える
*/
fn fibonacci_number(x_n: i32, x_n_1: i32) -> i32 {
    x_n + x_n_1
}