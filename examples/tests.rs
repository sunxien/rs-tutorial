/// return a + b
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
pub mod integrate_test_cases {
    /// `add` is a private function without `pub`
    /// integrate test module is a stand-alone module.
    /// `use create::add` or `use super::*` can resolve this problems.
    // use crate::add;
    use super::*;

    /// TODO before all test cases
    /// TODO after all test cases
    /// TODO run in each test cases
    #[test]
    pub fn test_assert() {
        assert_eq!(add(1, 1), 2);
    }

    /// Ignore this test case
    #[test]
    #[ignore]
    pub fn test_ignore() {
        panic!("ignore this test case....");
    }

    /// Error test case
    #[test]
    #[should_panic(expected = "oh my gosh....")]
    pub fn test_panic() {
        panic!("oh my gosh....");
    }
}


/// No `main` function found in crate `tests` [EO601]
fn main() {}