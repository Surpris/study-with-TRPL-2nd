/*
    title: モジュール
    name : Surpris
    date : 2020/02/02
*/

pub mod client; // ３．サブモジュールのインポート
                // ５－１．サブモジュールの公開
pub mod network; // ３．
                 // ５－４．公開

#[cfg(test)]
mod tests {
    use super::client; // １０．一つ上のモジュールを参照する
    #[test]
    fn it_works() {
        // ９－１．testsモジュールのスコープ内に client モジュールがないので、use 文がないとこれは動かない
        client::connect();

        // ９－２．`::` を頭につけてルートのスコープから持ってくる
        // ::client::connect()

        // ９－３．`super` を頭につけて一つ上のスコープから持ってくる
        super::client::connect();
        assert_eq!(2 + 2, 4);
    }
}
