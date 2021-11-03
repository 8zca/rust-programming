enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let a = Cons(5,
    //     Box::new(Cons(10,
    //         Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // 参照を共有する
    // Rc::cloneはディープコピーではなく参照を増やすこと(cloneという名前が紛らわしい)
    // aへの参照が残っている限り消えることはない
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // bは3->(a)->5->10
    let c = Cons(4, Rc::clone(&a)); // cは4->(a)->5->10
}
