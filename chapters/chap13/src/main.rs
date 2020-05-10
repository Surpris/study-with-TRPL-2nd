/*
    title: イテレータとクロージャ
    name : Surpris
    date : 2020/02/25
*/

use std::thread; // １．スレッドを扱うトレイト
use std::time::Duration; // ２．待ち時間を扱う構造体

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout_cacher(simulated_user_specified_value, simulated_random_number);
    println!("Hello, world!");

    let x = 4;
    let equal_to_x = |z| z == x;
    // ６．同じスコープの変数を参照するクロージャ
    /*
    fn equal_to_x(z: i32) -> bool {
        z == x
    } // 関数では外側のスコープの変数を参照することはできないためエラー
    */
    let y = 4;
    assert!(equal_to_x(y));

    // let equal_to_x = move |z| z == x; // ７．FnOnce トレイトを実装するクロージャ

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // ８－１．イテレータの生成

    for val in v1_iter {
        // ８－２．for ループでのイテレータの使用
        // 暗にイテレータの所有権がループに奪われ、可変に変換されている
        println!("Got: {}", val);
    }
}

/*
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    // 計算コストのかかる計算をする
    println!("calculating slowly...",);
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/

/*
fn simulated_expensive_calculation(num: u32) -> u32 {
    // 計算コストのかかる計算をする
    println!("calculating slowly...",);
    thread::sleep(Duration::from_secs(2));
    num
}
*/

#[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    // トレーニングプランを生成する
    let expensive_closure = |num| {
        // ４．クロージャを定義して変数に保存
        // "=" 以下がクロージャの定義に相当する
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // let expensive_result = simulated_expensive_calculation(intensity);
    // ３．計算コストのかかる関数の呼び出しを減らす最も単純な方法：変数に値を取り出しておく
    // この方法では関係のないスコープを通る場合も呼び出されてしまう
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));

        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn generate_workout_cacher(intensity: u32, random_number: u32) {
    // トレーニングプランを生成する
    let mut expensive_result = Cacher::new(|num| {
        // ５－３．クロージャ構造体インスタンスの生成
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let expensive_result = simulated_expensive_calculation(intensity);
    // ３．計算コストのかかる関数の呼び出しを減らす最も単純な方法：変数に値を取り出しておく
    // この方法では関係のないスコープを通る場合も呼び出されてしまう
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));

        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy,
{
    // ５－１．クロージャを含む構造体の定義
    // Copy を実装する U を返すすべてのクロージャに対して使用できる
    calculation: T,
    value: Option<U>, // ５－２．最初の呼び出し時に None が入ることを考慮してOptionで定義する
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy,
{
    fn new(calculation: T) -> Cacher<T, U> {
        // コンストラクタ
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: U) -> U {
        // 値を計算してキャッシュする
        // この実装だと二回目に別の値が入っても更新されない
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// TODO: Hashmap を使用した Cacher の改良
