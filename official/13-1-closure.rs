use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    //
    // 別のクロージャ
    //
    let example_closure = |x| x;

    // example_closureは初回呼び出し時点でメモ化されるためStringを返すように推論される
    // そのため2回めで別の型を受けるとエラーになる
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

// クロージャをキャッシュする構造体
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>,
    // value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
            // value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // valueがあればそれを返す, なければクロージャを実行して結果を保存し、その値を返す
        *self.value.entry(arg).or_insert((self.calculation)(arg))
        // match self.value {
        //     Some(v) => v,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     },
        // }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // クロージャ定義. Rubyっぽい
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        // 2s後に値を返す. 非同期ではないので連続で呼び出されても2sまって次の処理に映る
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
