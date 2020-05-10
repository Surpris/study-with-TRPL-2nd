/*
    title: 所有権
    name : Surpris
    date : 2020/01/29
*/

fn main() {
    let x = 5;
    let y = x; // ２．型のサイズが決まっているのでコピーされる
    println!("{}, {}", x, y);

    let s1 = String::from("new");
    let s2 = s1; // １．型のサイズが可変なのでムーブされる
                 // println!("{}, {}", s1, s2); //ムーブしているのでエラー
    println!("{}", s2);

    let s = String::from("hello");
    takes_ownership(s); // ３－１．可変長変数の場合、ムーブが起きるので s が無効になる。
                        // println!("{}", s); // ３－３．したがってこれはエラー

    let x = 5;
    makes_copy(x); // ４－１．固定長変数の場合、コピーが起きる

    let s1 = gives_ownership(); // ５－１．関数からムーブする
    println!("{}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // ５－２．引数にムーブして、さらに返す
    println!("{}", s3);

    let (s3, len) = calculate_length(s3); // ６－１．複数の戻り値
    println!("the length of `{}` is {}.", s3, len);

    let len = calculate_length_2(&s3); // ７－１．参照して計算。これにより引数の所有権は移らない。
    println!("the length of `{}` is {}.", s3, len);

    // let s = String::from("hello");
    // change(&s); // ８－１．借用した不変の変数の値は変更するができない

    let mut s = String::from("hello");
    change_2(&mut s); // ８－２．借用した可変の変数の値は変更することができる
    println!("{}", s);

    // let r1 = &mut s;
    // let r2 = &mut s; // ９．特定のスコープ内では、あるデータに対して可変な参照は一つしか持てない。

    // let r1 = &s;
    // let r2 = &s; // １０．不変の参照は複数行ってもよい
    // let r3 = &mut s; // １１．不変の参照が既にある状態で可変の参照は許可されない

    // let ref_to_nothing = dangle(); // １２－１．ダングリング参照は許可されない
    let s = no_dangle();
    println!("{}", s);

    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // １３－２．word には 5 が入っているが、s がクリアされると word 中の値は意味をなさない。
    println!("word: {}", word);

    let s = String::from("hello world");
    let word = first_word_2(&s); // １４－２．スライスが返ってくる。ここで不変として参照している
                                 // s.clear(); // １４－３．不変で参照しているので、元の String を更新する処理はエラーとなる
    println!("word: {}", word);

    // let s = "hello, world"; // １５．これは文字列リテラル &str

    let my_string = String::from("hello world");
    let word = first_word_3(&my_string[..]); // １６－２．String のスライスを入れる
    println!("word: {}", word);

    let my_string_literal = "hello world";
    let word = first_word_3(&my_string_literal[..]); // １６－３．文字列リテラルのスライスも引数にとれる
    println!("word: {}", word);

    let word = first_word_3(my_string_literal); // １６－４．文字列リテラル自体が文字列のスライスなので、そのまま引数にとれる
    println!("word: {}", word);

    let a = [1, 2, 3, 4, 5];
    let _slice = &a[..]; // １７．配列のスライス
}

fn takes_ownership(s: String) {
    //ムーブされる
    println!("{}", s);
} // ３－２．ここでスコープから抜けて drop が呼ばれ、メモリが解放される。

fn makes_copy(x: i32) {
    //コピーされる
    println!("{}", x);
} // ４－２．コピーが解放されるだけなので、元の変数に影響がない

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    // ６－２．複数の値を返す関数
    let length = s.len();
    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

// fn change(s: &String) {
//     s.push_str(", world");
// }

fn change_2(s: &mut String) {
    s.push_str(", world");
}

// fn dangle() -> &String { // １２－２．ダングリング参照を作ってしまう関数
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    // １２－３．直接 String を返せば、所有権が移るのでメモリが解放されない。
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> usize {
    // １３－１．スライスを使わずにインデックスを返してなんとかしようという例
    let bytes = s.as_bytes(); // バイト列に変換

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    // １４－１．スライスを返す
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_3(s: &str) -> &str {
    // １６－１．文字列スライスを引数にとることも可能
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
