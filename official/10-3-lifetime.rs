// fn longest(x: &str, y: &str) -> &str は戻り値の参照がどちらになるかコンパイラがわからないためエラー
// そのため 'a のライフタイム注釈を付与する
// すべての参照が 'a だけ生きることを指定している('からはじまれば何でもよい. 'aは慣習)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    // 最長の文字列は、{}です
    println!("The longest string is {}", result);
}
