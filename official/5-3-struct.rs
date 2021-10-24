// デバッグ用出力できるようにする
#[derive(Debug)]
struct Rect {
    width: usize,
    height: usize,
}

// メソッド記法
impl Rect {
    // 最初の引数はself (所有権を移さないように参照にしておく)
    // 型はRectのimple内のため自明(不要)
    // 変更が発生するなら &mut self になる場合も
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn is_contain(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect = Rect { width: 10, height: 5 };
    println!("{}", rect.area());

    let rect2 = Rect { width: 8, height: 4 };
    let rect3 = Rect { width: 9, height: 6 };
    println!("{}", rect.is_contain(&rect2));
    println!("{}", rect.is_contain(&rect3));
}
