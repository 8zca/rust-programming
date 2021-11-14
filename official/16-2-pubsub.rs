use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // multiple producer, single consumer = mpsc
    // 送信側と受信側がタプルで返る
    let (tx, rx) = mpsc::channel();

    // 送信側を増やす(受信側は同じrx)
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        // 送信メッセージ
        let val = String::from("hi");
        // moveでtxの所有権を奪う. sendで送信. Result<T, E>が返ってくるがunwrapでOKだけ扱う簡易実装
        tx.send(val).unwrap();
    });

    // recv()は値が返るまで待つ
    // 非同期で処理したいときは try_recv を定期的に呼んで受信処理する
    let received = rx.recv().unwrap();
    // Got: hi になる
    println!("Got: {}", received);


    //
    // rxへ複数のメッセージを送る
    //
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            // wait 1s
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recvを呼ばずにforで回すこともできる
    for received in rx {
        println!("Got: {}", received);
    }
}
