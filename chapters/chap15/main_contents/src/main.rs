/*
    title: スマートポインタ
    name : Surpris
    date : 2020/03/03
*/

use std::cell::RefCell; // RefCell 型の呼び出し
use std::ops::Deref; // ５－３．Deref トレイト
use std::rc::Rc; // ８－２．Rc 型の呼び出し
use std::rc::Weak; // Weak 型の呼び出し
use List::{Cons, Nil};
use ListRc::{Cons2, Nil2};
use ListRefCell::{Cons3, Nil3};
use ListRefCell2::{Cons4, Nil4};

fn main() {
    test_box();
    test_cons();
    test_deref();
    test_deref_box();
    test_deref_mybox();
    test_hello();
    test_drop();
    test_Rc();
    test_RefCell();
    test_circular_ref();
    test_node();
    test_node_2();
}

fn test_box() {
    let b = Box::new(5); // １．Box インスタンスの生成
    println!("b = {}", b);
}

enum List {
    // ２－１．コンスリスト
    // Cons(i32, List), // ２－２．List のサイズがわからないので失敗する
    Cons(i32, Box<List>), // ２－４．Box<T> はヒープに置かれる次の List 値を指している。また Box<T> の値を参照のように扱える
    Nil,
}

fn test_cons() {
    // let list = Cons(1, Cons(2, Cons(3, Nil))); // ２－３．再帰的な定義（失敗）
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); // ２－５．再帰的な定義
}

fn test_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // ３．参照は値と異なるので、中身を比較するときは参照外しが必要
}

fn test_deref_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // ４．Box はスマート__ポインタ__なので、値と比較するときは参照外しが必要
}

struct MyBox<T>(T); // ５－１．Box-like なタプル構造体

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        // ５－２．Deref トレイトを実装しないと参照の外し方が不明なのでエラーが出る
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // ５－４．Deref トレイトの実装
    type Target = T; // Deref トレイトが使用する関連型

    fn deref(&self) -> &T {
        // 内部メンバ（値）への参照を返す
        &self.0
    }
}

fn test_deref_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);
}

fn test_hello() {
    let name = Box::new(String::from("Bob"));
    println!("hello, {}", name); // ６．参照外し型強制
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // ７－１．Drop トレイトの実装
    fn drop(&mut self) {
        // ７－２．Drop トレイトの関連型
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn test_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
    // ７－３．この関数のスコープから外れるときに CustomSmartPointer のDrop 実装が呼び出される
    drop(c); // ７－４．強制的に解放させる
}

enum ListRc {
    Cons2(i32, Rc<ListRc>), // ８－１．Rc<T> による所有権の教諭
    Nil2,
}

fn test_Rc() {
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // ８－４．カウントを出力
    let b = Cons2(3, Rc::clone(&a)); // ８－３．Rc::cloneでは所有権ではなく参照を与える。
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum ListRefCell {
    // １３－１．Rc<RefCell<i32>> で可変化できる List を生成する
    Cons3(Rc<RefCell<i32>>, Rc<ListRefCell>),
    Nil3,
}

fn test_RefCell() {
    // １３－２．RefCell と Rc の組み合わせのテスト
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));

    let b = Cons3(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum ListRefCell2 {
    Cons4(i32, RefCell<Rc<ListRefCell2>>),
    Nil4,
}

impl ListRefCell2 {
    fn tail(&self) -> Option<&RefCell<Rc<ListRefCell2>>> {
        match *self {
            Cons4(_, ref item) => Some(item),
            Nil4 => None,
        }
    }
}

fn test_circular_ref() {
    let a = Rc::new(Cons4(5, RefCell::new(Rc::new(Nil4))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons4(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation= {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));

    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // １４－１．a に b を参照させる（循環参照）
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // １４－２．ここで b の参照が２になっていることがわかる
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // println!("a next item= {:?}", a.tail()); // １４－３．stack over flow する
}

#[derive(Debug)]
struct Node {
    // １５．Tree 構造
    value: i32,
    parent: RefCell<Weak<Node>>, // １６－１．Weak による親の情報の所有
    children: RefCell<Vec<Rc<Node>>>,
}

fn test_node() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // １６－２．downgrade() による弱い参照の生成

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // １６－３．upgrade() による Weak -> Rc への格上げ
}

fn test_node_2() {
    // １７．strong_count と weak_count への変更を可視化
    // 内側のスコープでbranchを作成し、強弱参照カウントを調査する
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // この段階では空の Vec への強参照のみある
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // 弱参照を用いた紐づけ
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // branch は leaf に対して強参照と弱参照で紐づいている
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        // branch が leaf 内の children に対する強参照を持つのでその分カウントが増加するが、弱参照はない
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    // スコープから外れたので branch が drop され parent の弱参照が外れる
    // もし強参照がスコープ内で生成されていた場合はそれも残る
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
