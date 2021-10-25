#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
        println!("call")
    }
}


fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{:?}", home); // V4(127, 0, 0, 1)
    home.call();

    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", loopback); // V6("::1")

    // Optionもenumとして定義されている
    // enum Option<T> { Some(T), None }
    let y: Option<i8> = Some(5);
}
