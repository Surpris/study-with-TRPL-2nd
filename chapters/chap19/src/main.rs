/*
    title: 高度な機能
    name : Surpris
    date : 2020/03/31
*/

use std::fmt;
use std::ops::Add; // Add trait

// ４．不変なグローバル変数の宣言
static HELLO_WORLD: &str = "Hello world!";
// ５．可変なグローバル変数の宣言
static mut COUNTER: u32 = 0;

fn main() {
    test_rew_pointer();

    unsafe {
        dangerous();
        let x: i32 = -3;
        println!("Absolute value of {} according to C is {}", x, abs(x));
    }

    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);
    // ５－３．可変な静的変数を読むことも unsafe である
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    test_trait_lifetime();
    test_add_point();
    test_full_path_call();
    test_full_path_call_2();
    test_super_trait();
    test_new_type_pattern();
    test_function_pointer();
    test_pointer_to_closure();
    test_returns_closure();
}

// １．生ポインタの生成テスト
fn test_rew_pointer() {
    println!("test_rew_pointer():");

    let mut num = 5;

    let r1 = &num as *const i32; // 参照から不変の生ポインタを生成
    let r2 = &mut num as *mut i32; // 参照から可変の生ポインタを生成
                                   // println!("r1 is {}", *r1); // safe スコープ内なのでエラー

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
}

// ２．unsafe な関数
unsafe fn dangerous() {
    println!("dangerous():");
}

// ３．C の標準ライブラリから abs 関数を呼び出す
// 常に unsafe の扱い
extern "C" {
    fn abs(input: i32) -> i32;
}

// ５－２．グローバル変数の値を変更する
fn add_to_count(inc: u32) {
    println!("add_to_count():");
    unsafe {
        COUNTER += inc;
    }
}

// ６－１．unsafe な trait
unsafe trait Foo {}

// ６－２．unsafe な trait の実装
unsafe impl Foo for i32 {}

trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

// ９．Trait のライフタイムに関する検証
fn test_trait_lifetime() {
    println!("test_trait_lifetime():");
    let num = 5;
    // num は 'static であり Red trait も特に注釈がなければ 'static である
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
    // println!("{:?}", obj);
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// １１．Add trait の実装による演算子オーバーロード
// trait Add<RHS=Self>: Self が規定の型となっている
impl Add for Point {
    type Output = Point;

    // add(self, rhs: RHS) -> Self::Output;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn test_add_point() {
    println!("test_add_point():");
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = Point { x: 3, y: 3 };
    assert_eq!(p3, p1 + p2);
}

// １３．フルパス記法
trait Pilot {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
    fn name() -> String {
        String::from("pilot")
    }
}

trait Wizard {
    fn fly(&self) {
        println!("Up!");
    }
    fn name() -> String {
        String::from("wizard")
    }
}

struct Human;

impl Pilot for Human {}
impl Wizard for Human {}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
    fn name() -> String {
        String::from("human")
    }
}

fn test_full_path_call() {
    println!("test_full_path_call():");
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

fn test_full_path_call_2() {
    println!("test_full_path_call_2():");
    println!("{}", Human::name());
    println!("{}", <Human as Pilot>::name());
    println!("{}", <Human as Wizard>::name());
}

// １４．スーパートレイト
// ここでは Display の機能を実装するトレイトを定義する
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Display の実装方法
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn test_super_trait() {
    println!("test_super_trait():");
    let p1 = Point { x: 3, y: 4 };
    p1.outline_print();
}

// １５．ニュータイプパターン
// Vec に Display を実装してみる
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn test_new_type_pattern() {
    println!("test_new_type_pattern():");
    let v = vec![String::from("hello"), String::from("world")];
    let w = Wrapper(v);
    // println!("v = {}", v); // Vec<T> は Display トレイトを実装していないのでエラー
    println!("w = {}", w);
}

// １６－１．型エイリアス
use std::marker::Send;
type Thunk = Box<dyn Fn() + Send + 'static>;

// １６－２．Result 型との組み合わせ
use std::io::Error;
// type Result<T> = Result<T, Error>; // すでに std::io の方で定義されているため、ここでは宣言できない

// １７．never 型を返す関数＝値が絶対に返されない関数
// fn bar() -> ! {
//     let x = 5; //TODO: 正しい実装
// }

// １８－１．関数ポインタとしての fn
fn add_one(x: i32) -> i32 {
    x + 1
}

// 関数ポインタを引数にとる関数
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn test_function_pointer() {
    println!("test_function_pointer():");
    let answer = do_twice(add_one, 5);

    println!("THe answer is {}", answer);
}

// １８－２．クロージャに関数ポインタを与える
fn test_pointer_to_closure() {
    println!("test_pointer_to_closure():");
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

// １９．クロージャをラップして戻り値とする
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn test_returns_closure() {
    println!("test_returns_closure():");
    let f = returns_closure();
    println!("{}", f(3));
}