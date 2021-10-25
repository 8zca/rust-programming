fn main() {
    // vとv2は同じ
    let mut v: Vec<usize> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v2 = vec![1, 2, 3];

    let third: &usize = &v2[2];
    // ないかもいしれない要素
    let fourth: Option<&usize> = v2.get(3);

    // loop
    for i in &v2 {
        println!("{}", i);
    }    

    // 複合型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
