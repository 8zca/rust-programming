fn main() {
    // a1: &str
    let a1 = "foo";
    // mut a2: &str 書き換える可能性があるときは mut String を使うらしいのでこれは使わないか
    let mut a2 = "foo2";
    println!("{}.{}", a1, a2);

    // &str は to_string() で String にできる
    let mut a: String = "abc".to_string();
    // String に &str を足すと String ができる
    // &str + String, String + String は不可
    a += "def";
    println!("{}", a); // => abcdef

    // .to_string() はどの型でも可. objectということか
    let x = 1.0.to_string();
    println!("{}", x); // 1

    // String を &str にするには as_str()
    a += x.as_str();
    println!("{}", a); // => abcdef1
}
