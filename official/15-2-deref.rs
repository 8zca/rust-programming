use std::ops::Deref;

fn main() {
    let x = 5;
    // 以下は同じ動き(dereference)
    let y = &x;
    let z = Box::new(x);
    let a = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *a);
}

struct MyBox<T>(T);

// Derefトレイトがないと&参照しか、参照外しできない
// Derefトレイトとderefメソッドで値をとって&参照を得られるようになる
// なのでL7,8は&をつけなくてもいい
impl<T> Deref for MyBox<T> {
    // 関連型
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
