enum List {
    // Listを再帰的にもつ
    // Cons(i32, List)はコンパイラが確保するメモリが不明になるためエラー
    // Box<T>とすればポインタをもてばいいことになるので自明になる
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
