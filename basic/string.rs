fn main() {
    // a1: &str
    let a1 = "foo";
    // mut a2: &str 書き換える可能性があるときは mut String を使うらしいのでこれは使わないか
    let mut a2 = "foo2";
    println!("{}.{}", a1, a2);

    // `&str`は`to_string()`メソッドで`String`にできる。
    let mut a: String = "abc".to_string();
    // 少しややこしいが、`String`に`&str`を足すと`String`ができる。
    // `&str`に`String`を足したり`String`に`String`を足したりはできない。
    a += "def";
    println!("{}", a); // => abcdef

    // `.to_string()`は様々な型に用意されている。
    let x = 1.0.to_string();
    println!("{}", x); // 1

    // `String`を`&str`にするには`as_str()`が使える。
    a += x.as_str();
    println!("{}", a); // => abcdef1
}
