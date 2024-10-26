extern crate core;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct Employee<'a> {
    emp_id: u32,
    emp_name: &'a str,
}

struct DebugEmployee<'a> {
    emp_id: u32,
    emp_name: &'a str,
}

struct DisplayEmployee<'a> {
    emp_id: u32,
    emp_name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct SystemError<'a> {
    code: u16,
    message: &'a str,
}

#[cfg(test)]
#[allow(unused, dead_code)]
pub mod traits_test_cases {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use serde::Serialize;
    use serde_json::{json, Serializer};
    use crate::{DebugEmployee, DisplayEmployee, Employee, SystemError};

    /// Other marker traits like: Send, Sync

    /// Clone trait
    #[test]
    pub fn test_clone_trait() {
        impl Clone for Employee<'_> {
            fn clone(&self) -> Self {
                println!("...............cloning...................");
                Employee { emp_id: self.emp_id, emp_name: self.emp_name }
            }
        }
        let emp = Employee { emp_id: 121001, emp_name: "xien.sxe" };
        println!("[test_clone_trait] {:?}, Addr: {:p}", &emp, &emp);

        let clone = emp.clone();
        println!("[test_clone_trait] {:?}, Addr: {:p}", &clone, &clone);
    }

    /// Display trait
    #[test]
    pub fn test_display_trait() {
        impl Display for DisplayEmployee<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                println!("...............displaying...................");
                write!(f, "编号：{}，姓名：{}", self.emp_id, self.emp_name)
            }
        }
        let emp = DisplayEmployee { emp_id: 121001, emp_name: "xien.sxe" };
        println!("[test_display_trait] {}, Addr: {:p}", &emp, &emp);
    }

    /// Debug trait
    #[test]
    pub fn test_debug_trait() {
        // conflicting implementation for `Employee<'_>`
        impl Debug for DebugEmployee<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                println!("...............do debug...................");
                write!(f, "编号：{}，姓名：{}", self.emp_id, self.emp_name)
            }
        }
        let emp = DebugEmployee { emp_id: 121001, emp_name: "xien.sxe" };
        println!("[test_debug_trait] {:#?}, Addr: {:p}", &emp, &emp);
    }

    /// Error trait
    #[test]
    pub fn test_error_trait() {
        impl Debug for SystemError<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_json::to_string(self).unwrap())
            }
        }
        impl Display for SystemError<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_json::to_string(self).unwrap())
            }
        }
        impl Error for SystemError<'_> {
            // ...
        }
        let system_err = SystemError { code: 500, message: "Internal Error" };
        println!("[test_error_trait] {:?}", system_err);
    }

    /// From trait
    #[test]
    pub fn test_from_trait() {
        // TODO 
        println!();
    }

    // Fn, FnOnce
}


/// No `main` function found in crate `traits` [EO601]
fn main() {}