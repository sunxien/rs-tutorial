#[cfg(test)]
pub mod move_semantics_test_cases {

    pub fn length(str: String) -> u32 {
        str.len()
    }

    pub fn test_move_semantics() {
        let x = String::from("abc");
        let len = length(x); // ownership move to `length` function
        println!("x: {:?}", len); // Compile error:
    }
}