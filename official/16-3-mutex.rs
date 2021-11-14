use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        // ロックを取得. ロック失敗したらunwrapでpanic!する
        let mut num = m.lock().unwrap();
        // 参照として取得できる
        *num = 6;
        // スコープの終わりとともにロック解除
    }

    println!("m = {:?}", m); // M = 6

    //
    // 1つのMutesを複数のスレッドで操作する
    //

    // L25でcounterがmoveされるので Mutex.new(0) のままではL33のスレッドで使えない => スマートポインタを使いましょう
    // ただし Rc<T> はスレッドセーフではないので Arc<T> を使う. Atomic RcでArc. スレッド使うときはArc使おう
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let c = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = c.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let c = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        let mut num2 = c.lock().unwrap();

        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
