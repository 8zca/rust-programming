// nまでの偶数の二乗の和
fn square_sum(n: isize) -> isize {
    // rubyぽく書けるが 0..nは nを含まないことに注意
    // コンパイル時に -O を指定することで最適化されるので1ループで済む
    // 境界を含む場合は 0..=n とも書ける. ...(ドット3つも同じ意味だが非推奨)
    (0..n + 1)
        .filter(|i| i % 2 == 0)
        .map(|i| i * i)
        .sum()
}

fn main() {
    println!("{}", square_sum(10));
}
