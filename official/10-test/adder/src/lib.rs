struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn raise_error() {
    panic!("error");
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// #[cfg(test)] は cargo test を実行したときだけコンパイルする. cargo buildには含まれない
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // panic!が発生することをテスト
    #[test]
    #[should_panic]
    fn raise_test() {
        raise_error();
    }
}
