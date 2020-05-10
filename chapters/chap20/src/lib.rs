/*
    title: マルチスレッドの Web server の構築
    name : Surpris
    date : 2020/04/15
*/

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex; // Mutex 型の呼び出し
use std::thread; // メッセージ転送で使用されるクレート // Arc 型の呼び出し

// １１－１．Box の中身をムーブするためのトレイトを定義
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    // Box<F> からクロージャ F をムーブして実行する
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// １０－１．thread::spawn に投げるクロージャを持つ Job 構造体
type Job = Box<dyn FnBox + Send + 'static>;

// １３－１．Job を持つ enum 型
enum Message {
    NewJob(Job),
    Terminate,
}

// ７－１．Thread pool の実装
pub struct ThreadPool {
    // ７－２．thread::spawn() が返すハンドルを格納するメンバー
    // ８－２．Worker にハンドルを格納する形式に変更
    workers: Vec<Worker>,
    // ９－２．Sender を持つ
    // Job -> Message による修正
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool instance.
    ///
    /// size: the number of workers in the pool
    ///
    /// # panic
    ///
    /// if size == 0 then the `new` method will panic.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // ９－１．Sender/Receiver でメッセージのやりとりを実装
        // Job -> Message による修正
        let (sender, receiver) = mpsc::channel::<Message>();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // Job -> Message による修正
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// １２－１．Drop トレイトの実装
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // １３－３．全ての Worker に Terminate メッセージを送る
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shuting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // １２－３．take() で thread の所有権をムーブする
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// ８－１．ハンドルを所持する Worker 構造体
struct Worker {
    id: usize,
    // １２－２．スレッドのハンドルをムーブできるように Option でラップする
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Job -> Message による修正
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // while let で実装すると job.call_box のロックが保持されたままになるため他の Worker が Job を受け取れなくなるらしい
        let thread = thread::spawn(move || {
            loop {
                // １０－２．receiver をロックしてから中身（Job）を取り出す
                // Job -> Message による修正
                let message = receiver.lock().unwrap().recv().unwrap();

                // １０－３．Job の中身を参照外しで取り出したいが、v1.39 のコンパイラではエラーが出る
                // (*job)();

                // １３－２．Message 列挙型を使用して match アームで受ける
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        // １１－２．FnBox で実装した call_box を呼ぶことで Job の中身をムーブして実行する
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
