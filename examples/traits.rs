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

pub trait Summary {
    fn summarize(&self) -> String;
}

struct Weibo {
    username: String,
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("this is weibo... username: {}", self.username)
    }
}

struct Post {
    author: String,
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("this is post... author: {}", self.author)
    }
}

#[cfg(test)]
#[allow(unused, dead_code)]
pub mod traits_test_cases {
    use std::cmp::Ordering;
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

    /// Trait `Eq` and `PartialEq`
    #[test]
    pub fn test_eq_and_partial_eq() {
        // float number is not implement `eq` trait as a special value `NaN`
        // The `NaN` can not be equals with each other!!!
        // Compile error: Trait `Eq` is not implemented for `f32` [E0277]
        // is_eq(3.14f32, 3.14f32);
        is_partial_eq(3.14f32, 3.14f32);
    }

    /// Trait `Eq`
    /// Example: fn is_eq<T: Eq + Debug>(a: T, b: T) {
    fn is_eq<T>(a: T, b: T)
    where
        T: Eq + Debug,
    {
        if a == b {
            println!("{:?} equals with {:?}? true", a, b)
        }
    }
    /// Trait `PartialEq`
    /// Example: fn is_partial_eq<T: PartialEq + Debug>(a: T, b: T) {
    fn is_partial_eq<T>(a: T, b: T)
    where
        T: PartialEq + Debug,
    {
        if a == b {
            println!("{:?} partial equals with {:?}? true", a, b)
        }
    }

    /// Trait `Add`
    #[test]
    pub fn test_add() {}

    /// Trait `Ord` and `PartialOrd`
    #[test]
    pub fn test_ord_and_partial_ord() {
        let mut floats = vec![3.0f32, 3.25f32, 3.5f32, 3.75f32, 4.0f32];
        // Compile error: Trait `Ord` is not implemented for `f32` [E0277]
        // floats.sort();

        let mut ints = vec![3, 4, 5, 6, 7];
        let r = ints.sort();
        for (i, v) in ints.iter().enumerate() {
            println!("vecs[{:?}]={:?}", i, v);
        }

        let mut s1 = Student::new(100001, "jack".to_string(), 98);
        let mut s2 = Student::new(100002, "pony".to_string(), 94);
        let mut s3 = Student::new(100003, "robin".to_string(), 99);
        let mut students = vec![s2, s1, s3];
        students.sort();
        for (i, v) in students.iter().enumerate() {
            println!("vecs[{:?}]={:?}", i, v);
        }
    }

    // #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[derive(Debug)]
    struct Student {
        id: u32,
        name: String,
        score: u16,
    }

    #[allow(dead_code, unused)]
    impl Student {
        pub fn new(id: u32, name: String, score: u16) -> Self {
            Self { id, name, score }
        }
    }

    impl Eq for Student {}

    impl PartialEq for Student {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }

        fn ne(&self, other: &Self) -> bool {
            self.id != other.id
        }
    }

    impl Ord for Student {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.score > other.score {
                Ordering::Greater
            } else if self.score < other.score {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }

        fn max(self, other: Self) -> Self
        where
            Self: Sized,
        {
            if self.score > other.score {
                self
            } else if self.score < other.score {
                other
            } else {
                self
            }
        }

        fn min(self, other: Self) -> Self
        where
            Self: Sized,
        {
            if self.score < other.score {
                self
            } else if self.score > other.score {
                other
            } else {
                self
            }
        }
    }

    impl PartialOrd for Student {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.score > other.score {
                Option::Some(Ordering::Greater)
            } else if self.score < other.score {
                Option::Some(Ordering::Less)
            } else {
                Option::Some(Ordering::Equal)
            }
        }

        fn lt(&self, other: &Self) -> bool {
            self.score < other.score
        }

        fn le(&self, other: &Self) -> bool {
            self.score <= other.score
        }

        fn gt(&self, other: &Self) -> bool {
            self.score > other.score
        }

        fn ge(&self, other: &Self) -> bool {
            self.score >= other.score
        }
    }

    ///
    #[test]
    pub fn test_drop() {
        let emp = Employee { emp_id: 121001, emp_name: "xien.sxe" };
        println!("{:?}", emp);

        let a = 1i32;
        let b = 2u64;
        let sum = a as u64 + b;
        println!("{:?} + {:?} = {:?}...Employee object has been dropped", a, b, sum);
    }

    impl Drop for Employee<'_> {
        fn drop(&mut self) {
            println!("drop employee object now....");
        }
    }
}


/// No `main` function found in crate `traits` [EO601]
fn main() {}