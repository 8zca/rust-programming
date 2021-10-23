fn add(x: usize) -> usize {
    // 値を返すなら ; 不要
    // https://qiita.com/tmshn/items/12f677d35a18251678c8
    x + 1
}

fn main() {
    println!("{}", add(1));
}
