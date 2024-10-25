#[cfg(test)]
pub mod options_test_cases {
    /// Option
    /// Rust Doc: https://course.rs/basic/match-pattern/option.html
    #[test]
    pub fn test_options1() {
        let none: Option<&str> = Option::None;
        let hello: Option<&str> = Option::Some("hello");
        if let Option::Some(v) = hello {
            println!("[test_options1] value is {:?}", v);
        } else {
            println!("[test_options1] None option!!!")
        }
    }

    /// Resolve ownership has moved problems.
    #[test]
    pub fn test_options2() {
        let hello: Option<String> = Option::Some(String::from("hello"));
        match hello {
            // value borrowed here after partial move, use `ref` keyword
            Some(ref v) => println!("[test_options2] {:?}", v),
            None => println!("none")
        }
        /// Instead codes like below:
        match &hello {
            // value borrowed here after partial move, use `ref` keyword
            Some(v) => println!("[test_options2] {:?}", v),
            None => println!("none")
        }
        println!("[test_options2] {:#?}", hello);
    }
}

/// No `main` function found in crate `options` [EO601]
fn main() {}