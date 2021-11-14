use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // スレッドの実行が終わるまで待つ.待たないとmainが終了した時点で子スレッドも強制終了
    // L11でjoinするとそこで待ち状態になる
    handle.join().unwrap();

    //
    // メインスレッドの編集を子で参照する
    //
    let v = vec![1, 2, 3];

    // moveをつけると強制的に（中で利用している）値の所有権を奪う
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();
}
