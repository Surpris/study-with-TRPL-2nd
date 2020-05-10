/*
    title: 入出力プロジェクトの開発
    subtitle: minigrep
    name : Surpris
    date : 2020/02/22
*/

extern crate minigrep; // １３－２．lib.rs に移したので外部クレートとして呼び出す

use std::env; // １．args() 関数を含むライブラリの呼び出し
use std::process; // ９．強制終了するためのトレイト

use minigrep::Config; // １３－３．Config 構造体を呼び出す

fn main() {
    // let args: Vec<String> = env::args().collect(); // ２．args() はイテレータなので collect 関数を呼び出して Vec にする

    // let query = &args[1]; // ３．args から必要な引数を参照して取り出す
    // let filename = &args[2];
    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    /*
    let config = Config::new(&args).unwrap_or_else(|err| {
        // unwrap_or_else() によって独自のエラー処理を定義
        eprintln!("Problem in parsing arguments: {}", err); // １８．エラーストリームへの出力マクロ
        process::exit(1);
    });
    */
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // １９－２．イテレータを用いた更新版を使用
        eprintln!("Problem in parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        // １２．Ok の場合は処理しないので、Err かどうかを確認して Err の場合に処理
        eprintln!("APplication error: {}", e);
        process::exit(1);
    }
}

/*
fn parse_config(args: &[String]) -> Config {
    // ４．config の取り出し関数を抽出
    // (&args[1], &args[2])
    Config {
        query: args[1].clone(), // String 型を保持するので参照ではなく clone でコピーを渡す
        filename: args[2].clone(),
    }
}
*/
