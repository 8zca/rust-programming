fn print(x: usize) {
    println!("{}", x);
}

fn main() {
    // 整数,boolean, 浮動小数点, char, 左記を含むタプルは所有権が移動しない(deep copy)
    let s1 = 10;
    let s2 = s1;

    // 呼べる
    println!("s1={}", s1);
    println!("s2={}", s2);

    print(s2);

    // 呼べる
    println!("s2={}", s2);
}
