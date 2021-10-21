fn main() {
    let x = 10;
    match x {
        0 => println!("0"),
        1 | 2 => println!("1 or 2"),
        1..=10 => println!("small number"),
        // 当てはまらない場合 n で受け取れる
        n => println!("big number: {}", n),
    }

    // タプル
    let t = (1, 2);
    match t {
        (0, 0) => println!("all zero"),
        // 部分一致も可
        (f, 0..=10) => println!("float: {} with small number", f),
        // 変数マッチしない場合は _ で無視
        _ => println!("other tuple"),
    }
}
