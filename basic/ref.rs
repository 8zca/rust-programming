fn main() {
    let x = 1;
    // &で参照
    let y = &x;
    println!("{}", y); // => 1

    let mut a = 1;
    // mutした変数の参照は &mut をつける
    let b = &mut a;
    // *参照 で代入できる. &mutならいつでも可能。
    *b = 2;

    // bがaを参照しているため、aを先に呼び出すことはできない(ダングリングポインタの恐れがある)
    // ライフタイム
    // println!("{}", a); // => 2
    println!("{}", b); // => 2
    println!("{}", *b); // => 2 (つまりa)
    println!("{}", a); // => 2
}
