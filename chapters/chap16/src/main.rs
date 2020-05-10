/*
    title: 並行性
    name : Surpris
    date : 2020/03/21
*/

use std::sync::mpsc; // ５．メッセージ転送で使用されるクレート
use std::sync::Mutex; // ９．Mutex 型の呼び出し
use std::sync::Arc; // １１．Arc 型の呼び出し
use std::thread; // １．スレッドクレート
use std::time::Duration;

fn main() {
    // test_thread();
    // test_join_thread();
    // test_move_value_thread();
    // test_channel();
    // test_multiple_messages();
    // test_multiple_transporter();
    test_mutex();
    test_arc_mutex();
}

// ２．スレッドの使用テスト
// メインスレッドが終了すると、別スレッドで実行しているプログラムも停止する仕様
fn test_thread() {
    println!("test_thread():");

    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi, number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi, number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// ３－１．スレッドハンドルのジョインテスト
fn test_join_thread() {
    println!("test_join_thread():");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi, number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // ３－３．ここに置いたらここでハンドルに対応するスレッドの終了を待っている
    // handle.join().unwrap();

    for i in 1..5 {
        println!("Hi, number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // ３－２．ここでハンドルに対応するスレッドの終了を待っている
    handle.join().unwrap();
}

// ４．元のスレッドの値を別スレッド内に移すテスト
fn test_move_value_thread() {
    println!("test_move_value_thread():");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    // println!("Here is a vector: {:?}", v); // すでに所有権が移っているのでエラー
    // drop(v); // これも上記と同じ理由でエラー
    handle.join().unwrap();
}

// ６－１．チャンネル使用テスト
fn test_channel() {
    println!("test_channel():");

    // ６－２．チャンネルの生成
    // tx: transporter
    // rx: receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");

        // ６－３．tx を別スレッドに移し、そこから rx にデータを転送する
        tx.send(val).unwrap();
    });

    // ６－４．rx からデータを取り出す
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// ７．複数のメッセージを転送する
fn test_multiple_messages() {
    println!("test_multiple_messages():");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// ８．複数の転送機から転送する
fn test_multiple_transporter() {
    println!("test_multiple_transporter():");

    let (tx, rx) = mpsc::channel();
    // clone でコピーを生成
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// １０．Mutex の使用テスト
fn test_mutex() {
    println!("test_mutex():");

    let m = Mutex::new(5);

    {
        // lock() によって値をロックする
        // このスコープから外れるまで他のスコープでの m の使用がブロックされる
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// １２．Arc + Mutex による Mutex 共有
// Arc は Rc と同じ API を持つ
fn test_arc_mutex() {
    println!("test_arc_mutex():");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Mutex を含む Arc のクローンを生成
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    //  参照外しによって counter 内の Mutex にアクセスする
    println!("Result: {}", *counter.lock().unwrap());
}