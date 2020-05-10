/*
    title: モジュール
    subtitle: 異なるモジュールの名前を参照する
    name : Surpris
    date : 2020/02/04
*/

use a::series::of; // ７－２．モジュールの参照
use a::series::of::nested_modules; // ７－４．関数も参照可能

// use TrafficLight::(Red, Yellow); // ８．列挙型の列挙子を参照
use TrafficLight::*; // ９．glob を使用する

fn main() {
    println!("Hello, world!");
    a::series::of::nested_modules(); // ７－１．フルパス指定で関数を呼び出す
    of::nested_modules(); // ７－３．参照された部分から関数を呼び出す
    nested_modules(); // ７－４．

    let _red = Red; // ８．
    let _yellow = Yellow; // ８．
    let _green = TrafficLight::Green; // ８．
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}
