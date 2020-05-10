/*
    title: スマートポインタ
    subtitle: RefCell<T>
    name : Surpris
    date : 2020/03/08
*/

pub trait Messenger {
    // ９．Messenger トレイト
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    // １０．値が最大値にどれくらい近いかを追跡し、特定のレベルの時に警告するライブラリ
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percent_of_max = self.value as f64 / self.max as f64;
        if percent_of_max >= 0.75 && percent_of_max < 0.9 {
            self.messenger
                .send("Warning: you've used up over 75% of your qouta!");
        } else if percent_of_max >= 0.9 && percent_of_max < 1.0 {
            self.messenger
                .send("Urgent warning: you've used up over 90% of your quota!");
        } else if percent_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // １１－１．借用チェッカーが許可してくれない MockMessenger を実装しようとする
        // ここでは RefCell の内部可変性を利用する
        // 外側の値は不変と考えられる一方でRefCell<T>で内部の値を可変化する
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.borrow_mut().push(String::from(message)); // １１－２．borrow_mut() によって RefCell の中身を mut で参照する
            let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut(); // １２．二回目の参照呼び出しは許可されない

            one_borrow.push(String::from(message));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(75);

        // limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); // １１－３．borrow() メソッドによって RefCell の中身を不変で参照する
    }
}
