/*
    title: 構造体
    name : Surpris
    date : 2020/01/31
*/

fn main() {
    let _user1 = User {
        // １－２．User 構造体のインスタンスを生成
        email: String::from("hoge@fuga.com"),
        username: String::from("hoge"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("hogehoge@fuga.com"),
        username: String::from("hogehoge"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("hogefuga@fuga.com"); // １－３．変更可能なインスタンスのフィールド値を変更

    let _user3 = build_user(String::from("user@example.com"), String::from("user"));

    let _user4 = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        active: user2.active, // ３－１．構造体の値を使用
        sign_in_count: user2.sign_in_count,
    }; // ？なぜか関数で生成された構造体 (user3) の変数を参照しようとするとエラーが出る

    let _user4 = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        ..user2 // ３－２．`..` により残りを参照
    };

    let _black = Color(0, 0, 0); // ４－２．タプル構造体インスタンスの生成
}

struct User {
    // １－１．構造体の定義の方法
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // ２－１．構造体を返す関数
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}

#[allow(dead_code)]
fn build_user_2(email: String, username: String) -> User {
    // ２－２．フィールドと引数の名前を同じにすると、省略記法が使える
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct Color(i32, i32, i32); // ４－１．タプル構造体
