/// If lifetime specifier (at return) is missing, error: Missing lifetime specifier
/// If lifetime specifier (at signature) is missing, error: Lifetime is undeclared
/// If lifetime specifier (at paramters) is missing, error: lifetime `'a` required
fn longest1<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b {
        a
    } else {
        b
    }
}

/// return the shortest lifetime
fn longest2<'a, 'b:'a>(a: &'a str, b: &'b str) -> &'a str {
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
pub mod lifetimes_test_cases {
    use crate::{longest1, longest2};

    /// Lifetime is only used for reference.
    /// `<'a>`, `<'static>`, and what is `<'_>` ?
    #[test]
    pub fn test_lifetimes1() {
        let str1 = String::from("hello");
        let str2: &str = "world".as_ref();
        let longest = longest1(&str1, str2);
        println!("longest str is {:?}", longest);
    }

    ///
    #[test]
    pub fn test_lifetimes2() {
        let str1 = String::from("hello");
        let str2: &str = "world".as_ref();
        let longest = longest2(&str1, str2);
        println!("longest str is {:?}", longest);
    }
}

/// No `main` function found in crate `lifetimes` [EO601]
fn main() {}