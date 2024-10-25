#[cfg(test)]
pub mod conditions_test_cases {
    /// if
    #[test]
    pub fn test_if() {
        for i in 1..11 {
            /// Rust Doc: https://docs.rs/rand/latest/rand/
            let num: i32 = rand::random();
            if num & 1 == 0 {
                println!("[test_if] No.{:?} It's an ODD number. ({:?})", i, num);
            } else {
                println!("[test_if] No.{:?} It's an EVEN number. ({:?})", i, num);
            }
        }
    }


    /// for
    #[test]
    pub fn test_for() {
        println!()
    }

    /// iterator
    #[test]
    pub fn test_iterator() {
        println!()
    }

    /// match
    #[test]
    pub fn test_match() {
        println!()
    }

    /// if let
    #[test]
    pub fn test_if_let() {
        println!()
    }

    // TODO Notice: while semantics is unsupported in Rust.
}

/// No `main` function found in crate `conditions` [EO601]
fn main() {}