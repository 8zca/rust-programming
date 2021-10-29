struct Point<T, U> {
    x: T,
    y: U,
}

// impleにも宣言が必要
impl<T, U> Point<T, U> {
    // 他方の型は<T,U>ではないことに注意
    // ジェネリクスの使い方は他の言語と同様
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
