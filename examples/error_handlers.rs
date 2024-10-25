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

#[cfg(test)]
pub mod error_handlers_test_cases {
    use std::num::ParseIntError;

    use rand::random;

    use crate::Response;

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
}

/// No `main` function found in crate `error_handlers` [EO601]
fn main() {}