/*
    title: エラー処理
    name : Surpris
    date : 2020/02/09
*/

use std::fs::File; // ３．ファイル入出力のトレイト
// use std::io::ErrorKind; // ４．エラーの種類の列挙型
use std::io::Read; // ６－１．ファイルからのデータの読み出しに必要なトレイト。 Implicit に使用されている

fn main() {
    // panic!("crash and burn"); // １．単純なパニックの呼び出し

    let v = vec![1, 2, 3];
    // v[99]; // ２－１．境界を超えるアクセスによる異常終了

    // let f: u32 = File::open("../data/hello.txt"); // ３－１．型が異なるようなエラーを起こすと、 <T, E> = <std::fs::File, std::io::Error> が返ってくる
    
    let file_path = "./data/hello.txt"; // ファイルが実行されるフォルダからの相対パスか、または絶対パス
    let f = File::open(file_path);

/*
    let f = match f {
        Ok(file) => file, // ３－２．Ok なら std::fs::File 型のハンドルが返ってくる
        Err(error) => { // ３－３．Err なら std::io::Error 型のインスタンスが返ってくる
            panic!("There was a problem opening hte file: {:?}", error);
        },
    };
*/

/*
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => { // ４－１．ファイルが見つからなかった場合の処理
            match File::create(file_path) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}.", e);
                },
            } // 値を返すのでセミコロンはつけない
        },
        Err(error) => { // ４－２．そのほかのエラーの処理
            panic!("There was a problem opening hte file: {:?}", error);
        },
    };
*/
    // let f = File::open(file_path).unwrap(); // ５－１．unwrap 関数の利用
    // let f = File::open(file_path).expect("Error"); // ５－２．expect 関数の利用
}

fn read_username_from_file() -> Result<String, std::io::Error> { // ７．Result<T, E> を返す関数の例
    let f = File::open("./data/hello.txt");
    let mut f = match f { // 後の処理のために mutable である必要がある
        Ok(file) => file,
        Err(e) => return Err(e), // return によって値を返して関数を終了する
    };

    let mut s = String::new();

    // ６－２．read_to_string() で暗に Read トレイトが使用されている
    // この関数は &self を第一引数にとるので、 f は mutable でなくてはならない
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } // 値を返すのでセミコロンはつけない
}

fn read_username_from_file_2() -> Result<String, std::io::Error> { // Result<T, E> を返す関数の例
    let mut f = File::open("./data/hello.txt")?; // ８．"?" 演算子の使用
    let mut s = String::new();
    f.read_to_string(&mut s)?; // ８．"?" 演算子の使用
    Ok(s)
}

fn read_username_from_file_3() -> Result<String, std::io::Error> { // Result<T, E> を返す関数の例
    let mut s = String::new();

    File::open("./data/hello.txt")?.read_to_string(&mut s)?; // ９．さらに短縮
    Ok(s)
}