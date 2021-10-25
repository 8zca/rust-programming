// 最初に書いても動く模様
use crate::front_of_house::hosting;
// 相対パスでuseする
// use self::front_of_house::hosting;

// 別名 import sample as ex from ..
use crate::front_of_house::sample as ex;

// useするとともに hosting をre-export
// pub use crate::front_of_house::hosting;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// fron_of_houseにpubがいらないのはhostingを呼び出しているeat_at_restaurantがpubかつfront_of_houseと兄弟要素だから
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod sample {
        pub fn foo() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    // 構造体自体の公開, メンバも個別に公開可否が選べる
    pub struct Breakfast {
        pub toast: String,
        // フルーツはシェフが決めるから選べない
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enumはpubをつければすべて公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    front_of_house::hosting::add_to_waitlist();

    // useしたので
    hosting::add_to_waitlist();

    // 朝食
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // enum参照
    back_of_house::Appetizer::Soup;
}

fn serve_order() {}
