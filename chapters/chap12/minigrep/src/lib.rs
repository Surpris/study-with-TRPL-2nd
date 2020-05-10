/*
    title: 入出力プロジェクトの開発
    subtitle: minigrep
    name : Surpris
    date : 2020/02/22
*/

// １３－１．main.rs から抽出

use std::env; // １７．var 関数を含むトレイト
use std::error::Error; // １１－１．Error トレイトの呼び出し
use std::fs::File; // ４．ファイルを扱うトレイト
use std::io::prelude::*; // ５．ファイル入出力に有用なトレイトを導入

pub struct Config {
    // ５．設定用構造体
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, // １６．大文字小文字を区別するかどうかのフラグ
}

impl Config {
    pub fn new_old(args: &[String]) -> Result<Config, &'static str> {
        // ６．parse_config の機能をコンストラクタとして定義
        // ８．Result 型で返すことで main で処理できるようにする
        if args.len() < 3 {
            // ７．引数が足りない場合のエラーを定義
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // is_err() は結果がエラーだったときに true を返す
        // env::var が環境変数 "CASE_INSENSITIVE" が定義されていない場合はエラーを返すため、これを利用している
        Ok(Config {
            query: args[1].clone(), // String 型を保持するので参照ではなく clone でコピーを渡す
            filename: args[2].clone(),
            case_sensitive: case_sensitive,
        })
    }

    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // １９－１．イテレータを使用した更新版コンストラクタ
        args.next(); // 最初の要素は実行ファイルのパスなのでスキップ
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file name."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // １０．ファイル処理の部分を抽出
    // １１－２．Result 型で返すように修正
    let mut f = File::open(config.filename)?; // ? はデフォルトのエラーを返す

    let mut contents = String::new();
    // ? はデフォルトのエラーを返す
    f.read_to_string(&mut contents)?;
    // println!("With text:\n{}", contents);

    let results = if config.case_sensitive {
        // １６－２．Config.case_sensitive で振り分ける
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[allow(dead_code)]
fn search_old<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // １４．返却されるベクタが contents の一部のスライスなので、ライフタイム注釈でそれを示している
    // vec![]
    let mut results = Vec::new();

    for line in contents.lines() {
        // 行ごとに処理
        if line.contains(query) {
            // query が含まれていれば処理
            results.push(line);
        }
    }

    results
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // １９－３．イテレータを用いた更新版の関数
    contents.lines().filter(|x| x.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // １５．大文字小文字の区別なく検索する
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            // to_lowercase() の戻り値が入っているので query は改めて参照することになる
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    #[ignore]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
