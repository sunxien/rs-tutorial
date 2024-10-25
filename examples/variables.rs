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

enum Message<'a> {
    CONNECT(&'a str),
    TRANSFER(u64, &'a str),
    SHUTDOWN(i8),
}

#[cfg(test)]
pub mod variables_test_cases {
    use std::any::type_name_of_val;

    use rand::random;

    use crate::{Employee, Message};

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
    #[test]
    pub fn test_signed_number() {
        let _i8: i8 = 1;
        let type_name = type_name_of_val(&_i8);
        let type_size = size_of_val(&_i8);
        println!("[test_signed_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _i16: i16 = 1;
        let type_name = type_name_of_val(&_i16);
        let type_size = size_of_val(&_i16);
        println!("[test_signed_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _i32: i32 = 1;
        let type_name = type_name_of_val(&_i32);
        let type_size = size_of_val(&_i32);
        println!("[test_signed_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _i64: i64 = 1;
        let type_name = type_name_of_val(&_i64);
        let type_size = size_of_val(&_i64);
        println!("[test_signed_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _i128: i128 = 1;
        let type_name = type_name_of_val(&_i128);
        let type_size = size_of_val(&_i128);
        println!("[test_signed_number] Size of {:?} is {:?} bytes", type_name, type_size);
    }

    /// f32, f64  [See also as: IEEE 754-2008 binary32, binary64]
    #[test]
    pub fn test_float_number() {
        /// Float types
        let _f32: f32 = 1.0;
        let type_name = type_name_of_val(&_f32);
        let type_size = size_of_val(&_f32);
        println!("[test_float_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _f64: f64 = 1.0;
        let type_name = type_name_of_val(&_f64);
        let type_size = size_of_val(&_f64);
        println!("[test_float_number] Size of {:?} is {:?} bytes", type_name, type_size);
    }

    /// i8, i16, i32, i64, i128, isize(>=16bits)
    /// assert_eq!(_u8, _u16); // Compile error: type mis-matched
    /// Notice: Max size of Object and Array is isize.MAX_VALUE
    #[test]
    pub fn test_unsigned_number() {
        let _u8: u8 = 1;
        let type_name = type_name_of_val(&_u8);
        let type_size = size_of_val(&_u8);
        println!("[test_unsigned_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _u16: u16 = 1;
        let type_name = type_name_of_val(&_u16);
        let type_size = size_of_val(&_u16);
        println!("[test_unsigned_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _u32: u32 = 1;
        let type_name = type_name_of_val(&_u32);
        let type_size = size_of_val(&_u32);
        println!("[test_unsigned_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _u64: u64 = 1;
        let type_name = type_name_of_val(&_u64);
        let type_size = size_of_val(&_u64);
        println!("[test_unsigned_number] Size of {:?} is {:?} bytes", type_name, type_size);

        let _u128: u128 = 1;
        let type_name = type_name_of_val(&_u128);
        let type_size = size_of_val(&_u128);
        println!("[test_unsigned_number] Size of {:?} is {:?} bytes", type_name, type_size);

        /// usize
        let _usize: usize = [1].len();
        let type_name = type_name_of_val(&_usize);
        let type_size = size_of_val(&_usize);
        println!("[test_number] Size of {:?} is {:?} bytes", type_name, type_size);
    }

    /// Use `as`, `TryInto` keyword. But these keywords only used for conversing.
    /// TODO Learn how to convert non-numeric types force!!!
    /// Rust Docs: https://course.rs/advance/into-types/converse.html
    #[test]
    pub fn test_number_convert() {
        // types conversions
        let _isize: isize = [1].len() as isize;
        let type_name = type_name_of_val(&_isize);
        let type_size = size_of_val(&_isize);
        println!("[test_number_convert] Size of {:?} is {:?} bytes", type_name, type_size);

        /// ................... usize, isize .........................
        let type_name = type_name_of_val(&usize::MIN);
        let type_size = size_of_val(&usize::MIN);
        println!("[test_number_convert] usize.MIN={:?}, usize.MAX={:?} bytes", usize::MIN, usize::MAX);

        let type_name = type_name_of_val(&isize::MIN);
        let type_size = size_of_val(&isize::MIN);
        println!("[test_number_convert] isize.MIN={:?}, isize.MAX={:?} bytes", isize::MIN, isize::MAX);

        let _isize: isize = usize::MAX as isize; // FIXME The result is -1 Overflow !!!!
        println!("[test_number_convert] usize.MAX={:?} as isize={:?}", usize::MAX, _isize);

        // FIXME unwrap can not be used in production env.(panic)
        // let _isize: isize = usize::MAX.try_into().unwrap();
        let _isize: isize = match usize::MAX.try_into() {
            Ok(v) => {
                println!("[test_number_convert] usize.MAX={:?} try_into isize={:?}", usize::MAX, v);
                v
            }
            Err(e) => {
                eprintln!("[test_number_convert] usize.MAX={:?} try_into isize={:?}. Error: {:?}", usize::MAX, -1, e.to_string());
                -1
            }
        };
    }

    /// char & str:
    /// char: unicode scalar value, [0x0000~0xD7FF,0xE000~0x10FFFF] 32bit unsigned char
    /// str: dynamic type, contains 8bit unsigned UTF-8 byte slice. (please use &str)
    /// Rust will auto deref &String to &str
    /// Please try to practice with other APIs
    #[test]
    pub fn test_string() {
        let s1 = String::from("hello"); // malloc on Heap
        println!("[test_string] value(String): {:?}", s1);

        let s2 = &s1; // &String means a reference to s1(String)
        println!("[test_string] value(&String): {:?}", s2);

        let s3 = &s1[0..s1.len()];
        println!("[test_string] value(&str): {:?}", s3); // &str means a slice reference
    }

    /// NeverType: !
    /// the `!` type is experimental
    #[test]
    pub fn test_never() {
        // let x: ! = panic!();
        // let y: u32 = x;
        // println!("Never type: {:?}, convert to u32: {:?}", x, y);
    }

    /// Tuple without elements like: `()` means `unit` or `unit type`
    #[test]
    pub fn test_tuple() {
        let unit = ();
        let type_name = type_name_of_val(&unit);
        let type_size = size_of_val(&unit);
        println!("[test_tuple] Size of {:?} is {:?} bytes", type_name, type_size);

        let tup = (121001, "xien.sxe", "xxxxxxxxx@gmail.com");
        let type_name = type_name_of_val(&tup);
        let type_size = size_of_val(&tup);
        println!("[test_tuple] Size of {:?} is {:?} bytes", type_name, type_size);
        println!("[test_tuple] tup.0={:?}, tup.1={:?}, tup.2={:?}", tup.0, tup.1, tup.2)
    }

    /// Array define: [T;N]
    #[test]
    pub fn test_array() {
        // alloc on Stack
        let arr = [1, 3, 5, 7, 9];
        for i in arr {
            println!("[test_array] alloc on stack arr[]={:?}", i)
        }
        // alloc on Heap
        let arr = Box::new([2, 4, 6, 8, 0]);
        for i in *arr /* or use arr.deref() */ {
            println!("[test_array] alloc on heap arr[]={:?}", i)
        }
    }

    /// Slice is a dynamic sized type.  [T]
    /// Slice types: &[T], &mut [T], Box<[T]>
    #[test]
    pub fn test_slice() {
        // array slice
        let arr_slice: Box<[i32]> = Box::new([2, 4, 6, 8, 0]);
        let type_name = type_name_of_val(&arr_slice);
        let type_size = size_of_val(&arr_slice);
        println!("[test_tuple] Size of {:?} is {:?} bytes", type_name, type_size);

        let slice_ref = &arr_slice[..];
        let type_name = type_name_of_val(&slice_ref);
        let type_size = size_of_val(&slice_ref);
        println!("[test_tuple] Size of {:?} is {:?} bytes", type_name, type_size);
    }

    /// Define a struct
    /// What is tuple struct? All fields are anonymous. e.g:  struct Location(u32, u32);
    /// What is unit-like struct? No fields are declared. e.g: struct Unit;
    #[test]
    pub fn test_struct() {
        let emp = Employee::new(121001, "xien.sxe");
        let to_string = emp.to_string();
        println!("{:?}", to_string);
    }

    ///
    #[test]
    pub fn test_enum() {
        let messages = [
            Message::CONNECT("v1.0.0"),
            Message::TRANSFER(11, "Hello World"),
            Message::SHUTDOWN(1)
        ];
        let length = messages.len() as u32;
        for i in 1..6 {
            let rand: u32 = random();
            let index = (rand % length) as usize;
            match messages[index] {
                Message::CONNECT(version) => {
                    println!("Connecting.... protocol={:?}", version)
                }
                Message::TRANSFER(byteSize, data) => {
                    println!("Receive {:?} bytes data: {:?}", byteSize, data)
                }
                Message::SHUTDOWN(exitCode) => {
                    println!("Program exit {:?}", exitCode)
                }
                _ => {
                    eprintln!("Unknown message!!!")
                }
            }
        }
    }

    ///
    #[test]
    pub fn test_union() {}

    /// All function-item type impl {Fn, FnMut, FnOnce, Copy, Clone, Send, Sync}
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
