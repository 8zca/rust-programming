// 整数型 i8, i16など
// 符号なし整数 u8, u16など
// isizeは適切な整数型を判断してくれる. 符号なしはusize
fn fizzbuzz(n: usize) {
    // 0..10 のレンジは10を含まないことに注意
    for i in 0..(n + 1) {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            // {}でフォーマット可
            println!("{}", i);
        }
    }
}

fn main() {
    fizzbuzz(20);
}
