fn main() {
    //
    // 例1
    //
    let x = 1;
    // &で参照
    let y = &x;
    println!("{}", y); // => 1

    //
    // 例2
    //
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

    //
    // 例3
    //
    let mut s = String::from("hello");
    // 可変参照はスコープにつき1回まで
    let r1 = &mut s;
    let r2 = &mut s;
    // これはエラー
    // println!("{},{}", r1,r2);

    //
    // 例4
    //
    let mut s = String::from("hello");
    // 不変参照はできる
    let r1 = &s;
    let r2 = &s;
    // 不変参照している間は可変参照できない
    let r3 = &mut s;
}
