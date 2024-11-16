

/// Rust Doc: https://course.rs/advance/macro.html
/// 1. declare macros
/// 2. procedural macros
#[cfg(test)]
#[allow(dead_code, unused_imports)]
pub mod macros_test_cases {

    /// 优点：1. 元编程；2. 可变参数；3. 宏展开；
    /// 缺点：1. 两个字：复杂；
    #[test]
    pub fn test_macro() {
        println!()
    }
}

/// No `main` function found in crate `macros` [EO601]
fn main() {}