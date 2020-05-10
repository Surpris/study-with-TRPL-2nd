extern crate adder; // １１．結合テストでは対象ライブラリを呼び出す必要がある

mod common; // １２－２．テストで共通利用する関数を含むモジュール

#[test]
fn it_adds_two() {
    common::setup(); // １２－３．共通利用する関数を呼び出す例
    assert_eq!(4, adder::add_two(2));
}
