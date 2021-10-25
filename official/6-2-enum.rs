enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: &Coin) -> u32 {
    // enum によるマッチ
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // stateはQuarterがとる UsState のいずれか
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Nickel;
    println!("{}", value_in_cents(&coin));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(&coin));
}
