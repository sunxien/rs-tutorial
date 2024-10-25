struct Employee<'a> {
    emp_id: u32,
    emp_name: &'a str,
}

/// Difference between `<'a>` and `<'_>` ?
impl Employee<'_> {
    pub fn new(emp_id: u32, emp_name: &str) -> Employee {
        Employee { emp_id: emp_id, emp_name: emp_name }
    }

    pub fn to_string(&self) -> String {
        format!("员工编号：{:?}，员工姓名：{:?}", self.emp_id, self.emp_name)
    }
}

#[cfg(test)]
pub mod variables_test_cases {
    use std::any::type_name_of_val;
    use crate::Employee;

    /// bool: 1 bit, false: 0x00, true: 0x01
    /// impl {Clone, Copy, Sized, Send, Sync}
    #[test]
    pub fn test_bool() {
        for i in 1..11 {
            /// Rust Doc: https://docs.rs/rand/latest/rand/
            let flag: bool = rand::random();
            println!("[test_bool] No.{:?} flag: {:?}", i, flag);
        }
    }

    /// u8, u16, u32, u64, u128, usize(>=16bits)
    /// i8, i16, i32, i64, i128, isize(>=16bits)
    /// f32, f64  [See also as: IEEE 754-2008 binary32, binary64]
    /// Notice: Max size of Object and Array is isize.MAX_VALUE
    #[test]
    pub fn test_number() {
        let _u8: u8 = 1;
        let type_name = type_name_of_val(&_u8);
        let type_size = size_of_val(&_u8);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _u16: u16 = 1;
        let type_name = type_name_of_val(&_u16);
        let type_size = size_of_val(&_u16);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _u32: u32 = 1;
        let type_name = type_name_of_val(&_u32);
        let type_size = size_of_val(&_u32);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _u64: u64 = 1;
        let type_name = type_name_of_val(&_u64);
        let type_size = size_of_val(&_u64);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _u128: u128 = 1;
        let type_name = type_name_of_val(&_u128);
        let type_size = size_of_val(&_u128);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        // TODO assert_eq!(_u8, _u16); // Compile error: type mis-matched

        let _i8: i8 = 1;
        let type_name = type_name_of_val(&_i8);
        let type_size = size_of_val(&_i8);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _i16: i16 = 1;
        let type_name = type_name_of_val(&_i16);
        let type_size = size_of_val(&_i16);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _i32: i32 = 1;
        let type_name = type_name_of_val(&_i32);
        let type_size = size_of_val(&_i32);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _i64: i64 = 1;
        let type_name = type_name_of_val(&_i64);
        let type_size = size_of_val(&_i64);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _i128: i128 = 1;
        let type_name = type_name_of_val(&_i128);
        let type_size = size_of_val(&_i128);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        /// Float types
        let _f32: f32 = 1.0;
        let type_name = type_name_of_val(&_f32);
        let type_size = size_of_val(&_f32);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _f64: f64 = 1.0;
        let type_name = type_name_of_val(&_f64);
        let type_size = size_of_val(&_f64);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);

        /// usize
        let array = [1];
        let _usize: usize = array.len();
        let type_name = type_name_of_val(&_usize);
        let type_size = size_of_val(&_usize);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);
    }

    #[test]
    pub fn test_string() {
        let s1 = String::from("hello"); // malloc on Heap
        println!("Value(String): {:?}", s1);

        let s2 = &s1; // &String means a reference to s1(String)
        println!("Value(&String): {:?}", s2);

        let s3 = &s1[0..s1.len()];
        println!("Value(&str): {:?}", s3); // &str means a slice reference

        // Rust will auto deref &String to &str
        // TODO Try to practice with other APIs
    }

    ///
    #[test]
    pub fn test_tuple() {}

    ///
    #[test]
    pub fn test_array() {}

    ///
    #[test]
    pub fn test_slice() {}

    /// Define a struct
    #[test]
    pub fn test_struct() {
        let emp = Employee::new(121001, "xien.sxe");
        let to_string = emp.to_string();
        println!("{:?}", to_string);
    }

    #[test]
    pub fn test_enum() {}

    #[test]
    pub fn test_union() {}

    #[test]
    pub fn test_function() {}

    #[test]
    pub fn test_closure() {}

    #[test]
    pub fn test_pointer() {}

    #[test]
    pub fn test_func_pointer() {}

    #[test]
    pub fn test_trait() {}
}


/// No `main` function found in crate `variables` [EO601]
fn main() {}
