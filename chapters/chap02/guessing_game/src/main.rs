/*
    title: 数当てゲーム
    url  : https://doc.rust-jp.rs/book/second-edition/ch02-00-guessing-game-tutorial.html
    name : Surpris
    date : 2020/01/29
*/

extern crate rand; // １．外部クレートの使用宣言

use std::io;
use std::cmp::Ordering; // ４．Ordering トレイトの呼び出し

use rand::Rng; // ２．外部クレート内の Rng トレイトの呼び出し。これがないとメソッドを使用できない

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // ３．rand::thread_rng は Rng トレイト内で定義されている

    // println!("THe secret number is: {}", secret_number);

    loop { // ８．永遠のループ
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // 既に guess は定義済みであるが、再利用できる。これを shadowing　という
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!"); // ７．trim()で両端を除去、parse()で数値に変換

        let guess: u32 = match guess.trim().parse() { // １１．.expect()の部分は match {} で例外処理が可能
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){ // ６．match 文で switch と同等の処理。すべての結果に対して処理を入れる必要がある
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { //９．複数の命令を書ける
                println!("You win!");
                break; // １０．break でループを抜ける
            },
        }
    }
}
