/*
    title: テストを書く
    subtitle: テスト関数の解剖
    name : Surpris
    date : 2020/02/21
*/

fn main() {}
#[derive(Debug)]
pub struct Rect {
    length: u32,
    width: u32,
}

impl Rect {
    pub fn can_hold(&self, other: Rect) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

pub fn greeting_invalid(name: &str) -> String {
    String::from("Hello")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn new_invalid(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn new_2(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            );
        }
        Guess { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    #[test] // １．テスト関数であることを示す
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exploration() {
        // ２．別名のテスト関数
        assert_eq!(2 + 4, 6);
    }

    #[test]
    #[ignore]
    fn another() {
        // ３．失敗するテストの例
        panic!("Make this test fail");
    }
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rect {
            length: 8,
            width: 7,
        };
        let smaller = Rect {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(smaller)); // ４．assert! マクロ
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        // ５．否定に対するアサート
        let larger = Rect {
            length: 8,
            width: 7,
        };
        let smaller = Rect {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(larger));
    }

    #[test]
    fn it_adds_two() {
        // ６．assert_eq! マクロによるテスト
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // let result = greeting_invalid("Carol"); // エラーが出る関数
        assert!(
            // ７．assert! に第二・第三引数を与える
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic] // ８．panic! が起こるとテストする
    fn greater_than_100() {
        // Guess::new_invalid(200); // バグあり
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")] // ９．expected 引数
    fn greater_than_100_expected() {
        Guess::new_2(200);
    }

    #[test]
    fn this_test_will_pass() {
        assert_eq!(prints_and_returns_10(4), 10);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        assert_eq!(prints_and_returns_10(5), 5);
    }

    #[test]
    fn internal() {
        // １０．非公開関数のテスト
        assert_eq!(internal_adder(2, 2), 4);
    }
}
