//! # My Crate
//! 全体の説明
/// Adds one to the number given.
/// 与えられた数値に1を足す。
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(7, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_one() {
        assert_eq!(add_one(1), 3);
    }
}
