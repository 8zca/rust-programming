use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_[name|value]はもう使えない
    // println!("{}", field_name);

    // アクセス
    let score = scores.get(String::from("Yellow"));
    println!("{:?}", score); // Some(10)

    // loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // hashになければ追加する
    // dig的なメソッド entry が使える
    scores.entry(String::from("Yellow")).or_insert(50);
}
