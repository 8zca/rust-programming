fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum();
    println!("{}", total);

    // イテレータに対するmapはcollectしないと処理されない
    // iterは参照、into_iterは値を使う=所有権がムーブされる
    // into_iterを使うとv3はそこで死ぬ
    let v3: Vec<i32> = vec![1, 2, 3];
    let v3_list: Vec<_> = v3.iter().map(|x| x + 1).collect();
    println!("{:?}", v3_list);

    let mut counter = Counter::new();
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// CounterにIteratorを実装
impl Iterator for Counter {
    type Item = u32;

    // nextが呼ばれたら1を足す(Max 5)
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
