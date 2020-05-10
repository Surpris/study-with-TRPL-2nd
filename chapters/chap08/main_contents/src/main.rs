/*
    title: コレクション
    name : Surpris
    date : 2020/02/04
*/

use std::collections::HashMap; // ２４－１．ハッシュマップのインポート

fn main() {
    let v: Vec<i32> = Vec::new(); // １．空のベクタの生成
    let v = vec![1, 2, 3]; // ２．マクロによるベクタの生成

    let mut v = Vec::new();
    v.push(5); // ３－１．値の追加
    v.push(6); // ３－２．既に格納されている値と同じ型を持つ物が追加できる

    {
        let vv = vec![1, 2, 3, 4];
    } // ４．スコープを抜けると Vec 型の変数に割り当てられたメモリも解放される

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // ５－１．値の参照
    let third: Option<&i32> = v.get(2); // ５－２．get() を用いる方が安全

    let mut v = vec![1, 2, 3, 5, 6];
    let first = &v[0];

    // v.push(6); // ６．不変借用の原則によりエラー

    println!("{}, {}", first, v[v.len() - 1]);

    let v = vec![1, 2, 3, 5, 6];
    for i in &v {
        // ７．for ループでの使用。参照を使う
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 5, 6];
    for i in &mut v {
        // ８－１．for ループで値を変える場合は &mut を使う
        *i += 50; // ８－２．参照変数に参照外し演算子をつける
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.2),
    ]; // ９．列挙型の使用により複数の型を持ったベクタ

    let mut s = String::new(); // １０．新規文字列の生成

    let data = "initial contents"; // １１．&strスライスの生成
    let s = data.to_string(); // １２－１．文字列リテラルを String に変換
    let s = String::from("initial contents"); // １２－１と等価
    let hello = String::from("こんにちは"); // １３．UTF8 でのエンコードなので日本語などほかの言語も対応している

    println!("{}, {}", s, hello);

    let mut s = String::from("foo");
    s.push_str("bar"); // １４－１．文字列へ文字列スライスを追加
    let s2 = "bar";
    s.push_str(s2); // １４－２．push_str() は引数に文字列とることが可能。このとき所有権を奪わない
    println!("s2 is {}", s2);

    s.push('l'); // １５－１．push は一文字（char）を追加する
    let c: char = 'f';
    s.push(c); // １５－２．char 型オブジェクトを引数にとれる
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // １６．"+"演算子による文字列の和。この場合s1はムーブされ、s2は借用される

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");
    // let s = s1 + "-" + &s2 + "-" + &s3; // １７．s1以外は &str 扱い

    let s = format!("{}-{}-{}", s1, s2, s3); // １８－１．format!マクロでの生成
    println!("{}", s1); // １８－２．format!の場合は所有権が移らない

    let s = String::from("hogehoge");
    // let h1 = s[0]; // １９－１．文字列はインデックスによるアクセスをサポートしていない

    let len = s.len(); // ２０－１．UTF8 での長さを返す
    println!("{}", len);

    let len = String::from("こんにちは").len(); // ２０－２．ひらがなは３バイト文字なので１５が返される
    println!("{}", len);

    let hello = "こんにちは"; // 文字列リテラル
    // let s = &hello[0..1]; // ２１－１．文字の切れ目でないため、癖巣はできるが後の処理でエラーが出る可能性がある
    // println!("{}", s);
    let s = &hello[0..6]; // ２１－２．文字の切れ目なのでOK
    println!("{}", s);

    for c in hello.chars() {
        // ２２．文字列の各要素にアクセス
        println!("{}", c);
    }

    for b in hello.bytes() {
        // ２３．文字列の各バイトにアクセス
        println!("{}", b);
    }

    let mut scores = HashMap::new(); // ２４－２．ハッシュマップオブジェクトの生成
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // ２５．collect() メソッドでハッシュマップを生成する

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // ２６－１．所有権が移る
    // println!("{}, {}", field_name, field_value);  // ２６－２．所有権が map へ移動しているため、呼び出しするとエラー

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // ２７．key を用いて対応する値を取り出す
    println!("The score of {} is {:?}", &team_name, score); // ２８．Some 型は Display を持たない

    for (key, value) in &scores {
        // ２９．for loop での利用
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // ３０．同じキーにインサートすると上書きされる

    scores.entry(String::from("Yellow")).or_insert(50); // ３１－１．キーに値があるかどうか確認し、なければ挿入する
    scores.entry(String::from("Blue")).or_insert(50); // ３１－２．すでに値があるので変更されない
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // ３２．Option<> は可変参照が返ってきているので、値の更新時に参照外しが必要
    }

    println!("{:?}", map);
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
