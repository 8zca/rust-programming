fn print(x: String) {
    println!("{}", x);
}

fn main() {
    let s1 = String::from("aaa");
    // 所有権を移動しないようにするには s1.clone(); を呼ぶ
    let s2 = s1;

    // s1は所有権が移動した
    // println!("{}", s1);
    println!("{}", s2);

    // スコープ内だから呼べる
    println!("{}", s2);

    // print関数に移動した
    print(s2);

    // もう呼べない
    // println!("{}", s2);
}
