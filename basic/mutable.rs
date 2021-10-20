fn rebind() {
    let sum = 0;
    for i in 0..10 {
        // スコープが違う
        let sum = sum + i;
    }
    println!("{}", sum); // => 0
}

fn reassign() {
    let mut sum = 0;
    for i in 0..10 {
        sum = sum + i;
    }
    println!("{}", sum); // => 45
}

fn main() {
    rebind();
    reassign();
}
