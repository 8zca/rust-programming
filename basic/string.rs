fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    // a1: &str 文字列リテラルはスタックに入る
    let a1 = "foo";
    // mut a2: &str 書き換える可能性があるときは mut String を使うらしいのでこれは使わない
    // なぜなら、Stringは可変かつ伸長可能なテキストを扱えるようにヒープに格納されるから
    let mut a2 = "foo2";
    println!("{}.{}", a1, a2);
    a2 = "foo3";
    println!("{}", a2);

    // &str は to_string() で String にできる
    // String::from("abc") でも同じことができる
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

    // push_strでも連結できる
    let mut s = String::from("やあ");
    s.push_str("こんにちわ");
    println!("{}", s);

    // pushでも連結できる
    let mut s2 = String::from("や");
    // pushは1文字(char)のみ(charなのでシングルクオート)
    s2.push('あ');
    println!("{}", s2);
}
