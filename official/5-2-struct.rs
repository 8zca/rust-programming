// デバッグ用出力できるようにする
#[derive(Debug)]
struct Rect {
    width: usize,
    height: usize,
}

fn main() {
    let rect = Rect { width: 10, height: 5 };
    // rectの所有権が移動してしまうので参照を渡す
    println!("{}", area(&rect));
    // スライスをフォーマットするときに使った :? を使える(#[derive(Debug)]と併用)
    println!("{:?}", rect);
}

fn area(rectangle: &Rect) -> usize {
    rectangle.width * rectangle.height
}
