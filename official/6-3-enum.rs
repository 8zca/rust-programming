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
    match coin {
        // {:?}州のクォーターコイン
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        _ => 1
    }
}

fn value_in_cents_if_let(coin: &Coin) -> u32 {
    // if let で代用できる
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        25
    } else {
        1
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(&coin));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents_if_let(&coin));
}
