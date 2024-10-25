extern crate core;

#[cfg(test)]
#[allow(unused, dead_code)]
pub mod traits_test_cases {
    use std::error;

    /// Other marker traits like: Send, Sync

    /// Clone trait
    #[test]
    pub fn test_clone_trait() {
        println!()
    }

    /// Debug trait
    #[test]
    pub fn test_debug_trait() {
        println!();
    }

    /// Display trait
    #[test]
    pub fn test_display_trait() {
        println!();
    }

    /// Error trait
    #[test]
    pub fn test_error_trait() {
        println!();
    }

    /// From trait
    #[test]
    pub fn test_from_trait() {
        println!();
        let l = Loc(22);
        l.sayHi();
        println!("{:#?}", l);
    }

    #[derive(Debug)]
    struct Bye<T> {
        value: T,
    }
    impl<T> Bye<T> {
        pub fn say(value: T) -> Self {
            Bye { value }
        }
        pub fn say<'a>(value: T) -> &'a Self {
            &Bye { value }
        }
    }
}


/// No `main` function found in crate `traits` [EO601]
fn main() {}