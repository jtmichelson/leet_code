/* Implement function ToLowerCase() that has a string parameter str,
and returns the same string in lowercase. */

impl Solution {
    pub fn to_lower_case(str: String) -> String {
        return str.chars().map( |it| {
            if it as u8 >= 65 && it as u8 <= 90 {
                (it as u8 + 32) as char
            } else {
                it
            }
        }).collect();
    }
}
