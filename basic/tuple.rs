fn main() {
    let a: (isize, f64, &str) = (1, 1.0, "abc");
    // タプル.index でアクセス
    println!("{}, {}, {}", a.0, a.1, a.2); // => 1, 1, abc
}
