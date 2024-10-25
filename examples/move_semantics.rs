#[cfg(test)]
pub mod macros_test_cases {
    /// Compile error: the parameter `str` takes ownership of the value
    pub fn length(str: String) -> usize {
        str.len()
    }

    #[test]
    pub fn test_move_semantics() {
        let str = String::from("abc");
        let len = length(str); // the ownership will move to `length` function
        println!("length of {:?} is {:?}", str, len); // Use `str` again, but occurred error
    }
}


/// No `main` function found in crate `move_semantics` [EO601]
fn main() {}