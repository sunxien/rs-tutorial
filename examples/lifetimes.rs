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
/// 'a:'b means 'a live longer than 'b
/// another usage: fn longest<'a, 'b>(a: &'a str, b: &'b str) -> &'a str where 'a:'b
fn longest2<'a, 'b: 'a>(a: &'a str, b: &'b str) -> &'a str {
    if a > b {
        a
    } else {
        b
    }
}

/// lifetime is defined in method
struct Employee<'a> {
    id: u32,
    name: &'a str,
}

impl<'a: 'b, 'b> Employee<'a> {
    fn test_for_lifetime_on_method(&'a self, ann: &'b str) -> &'a str {
        self.name
    }
}

///
#[derive(Debug)]
struct Foo {}
impl Foo {
    fn change_immut_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {
        println!("share now....")
    }
}

/// Rust Doc1: https://course.rs/basic/lifetime.html
/// Rust Doc2: https://course.rs/advance/lifetime/advance.html
#[cfg(test)]
pub mod lifetimes_test_cases {
    use crate::{Employee, Foo, longest1, longest2};

    /// Lifetime is only used for reference.
    /// `<'a>`, `<'static>`, and what is `<'_>` ?
    #[test]
    pub fn test_on_function_with_same_lifetimes() {
        let str1 = String::from("hello");
        let str2: &str = "world".as_ref();
        let longest = longest1(&str1, str2);
        println!("[test_on_function_with_same_lifetimes] longest str is {:?}", longest);
    }

    ///
    #[test]
    pub fn test_on_function_with_diff_lifetimes() {
        let str1 = String::from("hello");
        let str2: &str = "world";
        let longest = longest2(&str1, str2);
        println!("[test_on_function_with_diff_lifetimes] longest str is {:?}", longest);
    }

    /// Two usage: &'static, T: 'static
    #[test]
    pub fn test_on_method_with_diff_lifetimes() {
        let str1: &'static str = "world";
        let emp = Employee { id: 121001, name: "xien.sxe" };
        let result = emp.test_for_lifetime_on_method(str1);
        println!("[test_lifetimes_on_method] str is {:?}", result);
    }

    /// Two usage: &'static, T: 'static
    #[test]
    pub fn test_on_method_with_change_mutable() {
        let mut foo = Foo {};
        let r = foo.change_immut_and_share();
        r.share();
        println!("[test_on_method_with_change_mutable] r: {:?}", r);
    }
}

/// No `main` function found in crate `lifetimes` [EO601]
fn main() {}