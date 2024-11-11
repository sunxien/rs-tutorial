#[allow(dead_code)]
struct Response {
    code: u32,
    message: String,
    data: String, // TODO How to use `&str` with lifetime?
}

impl Response {
    /// 400, bad request
    pub fn failure<'a>(code: u32, message: String, data: String) -> Response {
        Response { code, message, data }
    }

    /// 200, "ok",
    pub fn success(code: u32, data: String) -> Response {
        Response { code, message: "ok".to_string(), data }
    }
}

/// Rust Doc1: https://course.rs/basic/result-error/panic.html
/// 1. ? can accept Option<T> or Result<T, E>
/// 2. impl std::error::Error for std::io::Error (From trait)
/// 3. try!(); is deprecated in the future!!!
///
/// Rust Doc2: https://course.rs/advance/errors.html
/// 1. or(), and(), and_or(),
#[cfg(test)]
pub mod error_handlers_test_cases {
    use std::fmt::{Debug, Display, Formatter};
    use std::{fs, io};
    use std::env::VarError;
    use std::io::Error;
    use std::num::ParseIntError;

    use rand::random;

    use crate::Response;

    /// Rust Doc: https://course.rs/basic/result-error/panic.html
    #[test]
    #[should_panic]
    fn test_panic() {
        let err = r#"
            Test panic!!!
            1. RUST_BACKTRACE=1 cargo run for Linux/MacOS.
            2. $env:RUST_BACKTRACE=1; cargo run for Windows.
        "#;
        panic!("{}", err);
    }

    /// Result
    #[test]
    pub fn test_error_handlers1() {
        #![allow(unused)]
        for i in 1..=5 {
            let age: i8 = random();
            let age_str = age.to_string();
            // FIXME creates a temporary value which is freed while still in use
            // let age_str = age.to_string().as_str();
            let result = if age > 100 {
                Result::Err(Response::failure(500, "too old".to_string(), age_str))
            } else if age < 0 {
                Result::Err(Response::failure(500, "invalid".to_string(), age_str))
            } else {
                Result::Ok(Response::success(200, age_str))
            };
            if let Result::Ok(resp) = result {
                println!("[test_error_handlers1] age: {:?}", resp.data);
            } else if let Result::Err(resp) = result {
                println!("[test_error_handlers1] wrong age: {:?}. Error: {:?}", resp.data, resp.message);
            } else {
                panic!("[test_error_handlers1] unexpected")
            }
        }
    }

    ///
    pub fn multiply(str: &str) -> Result<i32, ParseIntError> {
        // Compile error: Cannot multiply `Result<i32, ParseIntError>` by `i32` [E0369]
        // let multiply = str.parse::<i32>() * 1024;
        //
        let result = match str.parse::<i32>() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        // simple usage instead of match expression
        // let result = str.parse::<i32>()?;
        Ok(result * 2)
    }

    /// Result
    #[test]
    pub fn test_error_handlers2() {
        let str = "123";
        let result = multiply(str);
        println!("[test_error_handlers2] result is {:?}", result);
    }

    /// Rust Doc: https://course.rs/advance/errors.html
    #[test]
    pub fn test_or_and() {
        //
        let opt1: Option<i32> = Some(1);
        let opt2: Option<i32> = Some(2);
        let none1: Option<i32> = None;
        let none2: Option<i32> = None;
        println!("{:p}, {:p}", &none1, &none2);

        // either of Option is true
        println!("{:?} or {:?} is {:?}", opt1, opt2, opt1.or(opt2));
        println!("{:?} or {:?} is {:?}", opt2, opt1, opt2.or(opt1));
        println!("{:?} or {:?} is {:?}", opt1, none1, opt1.or(none1));
        println!("{:?} or {:?} is {:?}", none1, opt1, none1.or(opt1));
        println!("{:p} or {:p} is {:?}", &none1, &none2, none1.or(none2));

        // both of Options are true
        println!("{:?} and {:?} is {:?}", opt1, opt2, opt1.and(opt2));
        println!("{:?} and {:?} is {:?}", opt2, opt1, opt2.and(opt1));
        println!("{:?} and {:?} is {:?}", opt1, none1, opt1.and(none1));
        println!("{:?} and {:?} is {:?}", none1, opt1, none1.and(opt1));
        println!("{:?} and {:?} is {:?}", &none1, &none2, none1.and(none2));

        let res1: Result<i32, &str> = Ok(1);
        let res2: Result<i32, &str> = Ok(2);
        let err1: Result<i32, &str> = Err("error1 occurred");
        let err2: Result<i32, &str> = Err("error2 occurred");

        // either of Result<T,E> is true
        println!("{:?} or {:?} is {:?}", res1, res2, res1.or(res2));
        println!("{:?} or {:?} is {:?}", res2, res1, res2.or(res1));
        println!("{:?} or {:?} is {:?}", res1, err1, res1.or(err1));
        println!("{:?} or {:?} is {:?}", err1, res1, err1.or(res1));
        println!("{:?} or {:?} is {:?}", err1, err2, err1.or(err2));

        // both of Result<T,E> are true
        println!("{:?} and {:?} is {:?}", res1, res2, res1.and(res2));
        println!("{:?} and {:?} is {:?}", res2, res1, res2.and(res1));
        println!("{:?} and {:?} is {:?}", res1, err1, res1.and(err1));
        println!("{:?} and {:?} is {:?}", err1, res1, err1.and(res1));
        println!("{:?} and {:?} is {:?}", err1, err2, err1.and(err2));
    }

    /// Rust Doc: https://course.rs/advance/errors.html
    #[test]
    pub fn test_orelse_andthen() {}

    ///
    #[test]
    pub fn test_mapor_maporelse() {
        {
            const DEFAULT_VALUE: u8 = 1;
            let res1: Result<u8, ()> = Ok(10);
            let none: Option<u8> = None;
            let mapped1 = res1.map_or(DEFAULT_VALUE, (|v: u8| v + 2));
            println!("{:?} map_or ({:?}, |v: u8| v + 2) is {:?}", res1, DEFAULT_VALUE, mapped1);

            let mapped2 = none.map_or(DEFAULT_VALUE, (|v: u8| v + 2));
            println!("{:?} map_or ({:?}, |v: u8| v + 2) is {:?}", res1, DEFAULT_VALUE, mapped2);
        }

        {
            // map_or_else  (default_val_closure, fn_closure)
            let some1 = Some(10);
            let none: Option<u8> = None;

            let default_closure = || 1;
            let compute_closure = |v: u8| v + 2;
            let mapped1 = some1.map_or_else(default_closure, compute_closure);
            println!("{:?} map_or_else (|| 1, |v: u8| v + 2) is {:?}", some1, mapped1);

            let mapped2 = none.map_or_else(default_closure, compute_closure);
            println!("{:?} map_or_else (|| 1, |v: u8| v + 2) is {:?}", none, mapped2);
        }
    }

    /// Custom Error
    #[derive(Debug)]
    struct AppError {
        code: u32,
        message: String,
    }
    impl AppError {
        pub fn new(code: u32, message: String) -> Self {
            AppError { code, message }
        }
    }
    impl Display for AppError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Code: {}\nMessage:{}", self.code, self.message)
        }
    }
    impl std::error::Error for AppError {
        // should implement source(&self)???
        // if this error has child errors, u must rewrite source(&self)
    }

    /// Q: 逻辑中可能抛出不同类型的异常，如何归一处理？
    /// 1. 使用 Box<dyn Error>, all errors must implement std::error::Error
    pub fn return_errors1() -> Result<String, Box<dyn std::error::Error>> {
        let f = std::env::var("MARKDOWN")?; // return VarError
        let source = fs::read_to_string(f)?; // return std::io::Error
        Ok(source)
    }
    /// 2. 自定义错误类型
    struct MyError {
        message: String,
    }
    impl Debug for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "错误：{}", self.message)
        }
    }
    impl Display for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "错误：{}", self.message)
        }
    }
    impl std::error::Error for MyError {
        // should implement source(&self)???
        // if this error has child errors, u must rewrite source(&self)
    }
    impl From<std::env::VarError> for MyError {
        fn from(value: VarError) -> Self {
            MyError { message: value.to_string() }
        }
    }
    impl From<std::io::Error> for MyError {
        fn from(value: Error) -> Self {
            MyError { message: value.to_string() }
        }
    }
    pub fn return_errors2() -> Result<String, MyError> {
        let f = std::env::var("MARKDOWN")?; // return VarError
        let source = fs::read_to_string(f)?; // return std::io::Error
        Ok(source)
    }
    /// 3. 使用 thiserror: https://course.rs/advance/errors.html
    #[test]
    pub fn test_custom_error() {
        let r: Result<(), AppError> = Result::Err(AppError::new(404, String::from("Not Found")));
        match r {
            Err(e) => {
                eprintln!("{}", e);
            }
            _ => panic!("No errors")
        }
        let r1 = return_errors1();
        let r2 = return_errors2();
    }
}

/// No `main` function found in crate `error_handlers` [EO601]
fn main() {}