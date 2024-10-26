#[cfg(test)]
pub mod lifetimes_test_cases {

    /// Lifetime is only used for reference.
    /// `<'a>`, `<'static>`, and what is `<'_>` ?
    #[test]
    pub fn test_lifetimes1() {
        println!()
    }
}

/// No `main` function found in crate `lifetimes` [EO601]
fn main() {}