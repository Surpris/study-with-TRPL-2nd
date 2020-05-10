/*
    title: モジュール
    subtitle: プライバシー例
    name : Surpris
    date : 2020/02/04
*/

mod outermost {
    // ６－１．プライバシー例のテストモジュール
    pub fn middle_function() {} // ６－２．親（今回はルートモジュール）からアクセスできる

    fn middle_private_function() {} // ６－３．非公開だが親からアクセス可能

    mod inside {
        // ６－４．非公開だと親の親からアクセスできない
        pub fn inner_function() {}
        fn private_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // outermost::middle_private_function(); // これはエラー
    // outermost::inside::inner_function(); // これはエラー
    // outermost::inside::private_function(); // これはエラー
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
