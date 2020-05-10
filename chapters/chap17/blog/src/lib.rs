/*
    title: オブジェクト指向プログラミング
    name : Surpris
    date : 2020/03/29
    exercise: 
        1. 記事の状態を PendingReview から Draft に戻す reject メソッドを追加する
        2. 状態が Published に変化させられる前に approve を２回呼び出す必要があるようにする
        3. 記事が Draft 状態のときのみテキスト内容をユーザーが追加できるようにする。
*/

pub struct Post {
    state: Option<Box<dyn State>>, // State トレイトを実装したオブジェクトを持てる
    content: String,
}

impl Post {
    // 初期化メソッド
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // テキスト追加メソッド
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // メンバーと同名のメソッド。content を返す予定
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    // 記事の査読を要求するメソッド
    pub fn request_review(&mut self) {
        // take() は所有権を移す
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // 状態が承認されたときに現在の状態をセットするメソッド
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// 異なる記事の状態で共有される振る舞いを定義するトレイト
trait State {
    // 記事の査読を要求する抽象的なメソッド
    // 型を保持する Box に対して呼ばれたときのみ実行可能
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    // 状態が承認された時に現在の状態をセットする抽象的なメソッド
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // content を返す抽象的なメソッド
    // 返される値は第二引数から持ってくるので、第二引数と同じライフタイムを明記する
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) ->&'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
