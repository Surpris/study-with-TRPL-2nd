/*
    title: イテレータとクロージャ
    name : Surpris
    date : 2020/02/25
*/

#[test]
fn iterator_demonstration() {
    // ９．イテレータのテスト
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1)); // １０．next() を用いるとイテレータの要素が消費される
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum(); // １１．消費アダプタの例。sum()は格納先の型を指定する必要がある。これは sum() が返すのが Some であるため？
    assert_eq!(total, 6);
}

#[test]
fn iterator_map_collect() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // １２．イテレータにクロージャを適用して別のベクタを生成
    // 返ってくるものは型が不定なので推定させるか与える必要がある。
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // １３．filter メソッドによるフィルタリング
    // 環境の変数である shoe_size をクロージャ内でキャプチャしている
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    // １３を使用した関数のテスト
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let my_size: u32 = 10;
    let in_my_size = shoes_in_my_size(shoes, my_size);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // １４－１．Iterator トレイトの実装
    type Item = u32; // １４－２．type Item を定義

    fn next(&mut self) -> Option<Self::Item> {
        // １４－３．next メソッドの実装
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    // １４－４．Counter イテレータのテスト
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    // １４－５．イテレータメソッドのテスト
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
