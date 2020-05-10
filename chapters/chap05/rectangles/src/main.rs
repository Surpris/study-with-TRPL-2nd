/*
    title: 構造体
    subtitle: 構造体を使ったプログラムの例
    name : Surpris
    date : 2020/01/31
*/

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("{}", area(width1, height1)); // １．

    let rect1 = (width1, height1);
    println!("{}", area_2(rect1)); // ２．

    let rect2 = Rect {
        width: width1,
        height: height1,
    };
    println!("{}", area_3(&rect2)); // ３．

    println!("rect2 is {:?}", rect2); // ４－２．デバッグ出力
    println!("rect2 is {:#?}", rect2); // ４－２．デバッグ出力の別の表現

    println!("The area of rect2 is {}", rect2.area()); // ５－２．メソッドの呼び出し

    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    println!("The distance is {:1.3}", p1.distance(&p2));
    println!("The distance is {:1.3}", (&p1).distance(&p2)); // ６．これらは同じ動作

    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    let rect3 = Rect {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(width: u32, height: u32) -> u32 {
    // １．矩形の面積を計算
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    // ２．タプルでリファクタリング
    dimensions.0 * dimensions.1
}

#[derive(Debug)] // ４－１．デバッグ出力用の derive 注釈。Debug トレイトを継承している
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        // ５－１．メソッドの定義
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        // ７．複数の引数を持つ
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rect {
        // ８．&self を引数にとらない関数も定義可能
        Rect {
            width: size,
            height: size,
        }
    }
}

fn area_3(rect: &Rect) -> u32 {
    // ３．構造体を用いたリファクタリング
    rect.width * rect.height
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);
        f64::sqrt(x_squared + y_squared)
    }
}

impl Point {
    // ９．別の impl ブロックを用いたメソッドの定義が可能
    fn deviation(&self, other: &Point) -> (f64, f64) {
        (other.x - self.x, other.y - self.y)
    }
}
