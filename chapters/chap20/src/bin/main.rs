/*
    title: マルチスレッドの Web server の構築
    name : Surpris
    date : 2020/04/14
*/

use std::fs::File;
use std::io::prelude::*; // ２－２．TCP stream の読み書きを行うので必要なトレイトを呼び出す
use std::net::TcpListener; // １－１．TCP リッスン用の構造体
use std::net::TcpStream; // ２－１．TCP stream 構造体 // 一般的なファイルストリーム構造体
extern crate chap20;
use chap20::ThreadPool;

fn main() {
    test_tcp_listener();
}

// １－２．TcpListener のテスト
fn test_tcp_listener() {
    println!("test_tcp_listener()");

    let addr: &str = "127.0.0.1:7878";
    // 指定されたアドレスに TCP 接続するためのインスタンスを生成
    let listener = TcpListener::bind(addr).unwrap();
    println!("Wait for connection...");

    let pool = ThreadPool::new(4);

    // 指定されたアドレスからのストリームを取得して処理する
    let mut is_connected = false;
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        if is_connected == false {
            println!("Connection to {} established!", &addr);
            is_connected = true;
        }

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// ６－１．リファクタリングされた handle_connection
fn handle_connection(mut stream: TcpStream) {
    // 0 で構成される 512 バイトのバッファ配列を用意
    let mut buffer = [0; 512];
    // TCP stream からのデータの読み出しを行う
    stream.read(&mut buffer).unwrap();
    // 特定のリクエスト（今回は `/`）に対応する get 文
    let get = b"GET / HTTP/1.1\r\n";

    // ６－２．let if 文で status_line, file_path を入れる
    let (status_line, file_path) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", ".\\html\\hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", ".\\html\\404.html")
    };
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[allow(dead_code)]
fn handle_connection_old(mut stream: TcpStream) {
    // 0 で構成される 512 バイトのバッファ配列を用意
    let mut buffer = [0; 512];

    // ２－３．TCP stream からのデータの読み出しを行う
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // ５－１．特定のリクエスト（今回は `/`）に対応する get 文
    let get = b"GET / HTTP/1.1\r\n";

    // ５－２．buffer の開始が `/` への get であるときの処理
    if buffer.starts_with(get) {
        // ４．HTML の中身を返す
        let fpath: &str = ".\\html\\hello.html";
        let mut file = File::open(fpath).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        // ３．レスポンスを書き込むためのヘッダー
        // 今回は成功のレスポンスを返す
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        // flush() は write の終了まで待機させる
        stream.flush().unwrap();
    } else {
        // ５－３．ほかのリクエストへの get の場合の処理
        let fpath: &str = ".\\html\\404.html";
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open(fpath).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
