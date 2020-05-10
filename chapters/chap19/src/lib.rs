/*
    title: 高度な機能
    name : Surpris
    date : 2020/04/01
*/

use std::ops::Add;

struct Context<'a>(&'a str);

// ７．ライフタイム・サブタイピング
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

// ８－１．ライフタイム境界
// T は Ref よりも長生きする必要があることを要求する
struct Ref<'a, T: 'a>(&'a T);

// ８－２．'static ライフタイム境界による制限
// T はプログラム全体と同じだけのライフタイムを持つことを要求する
struct StaticRef<T: 'static>(&'static T);

// １０．関連型 Item がある Iterator トレイト
// 13 章ですでに扱っている
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

//　１２．trait のジェネリクスの規定の型を変更する
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
