#[cfg(test)]
pub mod unsafes_test_cases {
    use std::ptr::{addr_of, addr_of_mut};
    use std::slice;
    use std::slice::from_raw_parts;
    use std::str::from_utf8_unchecked;

    use rocket::form::validate::Len;

    /// Rust Doc: https://course.rs/advance/unsafe/intro.html
    /// 1. deref raw pointer
    /// 2. use unsafe or call external function
    /// 3. impl unsafe trait
    /// 4. use fields defined in `union`
    #[test]
    pub fn test_unsafe_ptr_of_addr() {
        let mut num = 5;

        unsafe {
            let addr_ptr = &num as *const i32;
            println!("addr_ref: {:p}, addr_ptr: {:?}, value: {}", &num, addr_ptr, *addr_ptr);
        }

        let addr_of = addr_of!(num); // for `*const`
        unsafe {
            println!("addr_ref: {:p}, addr_of: {:?}, value: {}", &num, addr_of, *addr_of);
        }

        let addr_of_mut = addr_of_mut!(num); // for `*mut`
        unsafe {
            println!("addr_ref: {:p}, addr_of_mut: {:?}, value: {}", &num, addr_of_mut, *addr_of_mut);
        }
    }

    /// `*const T`, `*mut T` maybe exists
    /// invalid addr
    /// maybe `null`
    /// not impl `drop` trait
    #[test]
    pub fn test_deref_raw_ptr() {
        let mut num = 3;

        // create `*const T`, `*mut i32` is safe
        let ptr1 = &num as *const i32;
        let ptr2 = &mut num as *mut i32;

        // but deref them is unsafe...
        println!("ptr(*const i32): {:?}, ptr(*mut i32): {:?}", ptr1, ptr2);
    }

    #[test]
    pub fn test_create_raw_ptr() {
        let addr = 0x700003eda7f4usize;
        let r = addr as *const i32;
        // maybe 1. undefined behavior; 2. segmentation fault;
        unsafe {
            // signal: 11, SIGSEGV: invalid memory reference
            println!("value: {:?}", *r);
        }
    }

    ///
    pub fn get_memory_addr() -> (usize, usize) {
        let str = "Hello World";
        let length = str.len();
        (str.as_ptr() as usize, length)
    }
    pub fn get_str_at(ptr: usize, len: usize) -> &'static str {
        unsafe {
            from_utf8_unchecked(from_raw_parts(ptr as *const u8, len))
        }
    }

    #[test]
    pub fn test_create_certain_raw_ptr() {
        let (ptr, len) = get_memory_addr();
        let str = get_str_at(ptr, len);
        println!("ptr: {:?}, len: {:?}, str: {:?}", (ptr as *const u8), len, str);

        // signal: 11, SIGSEGV: invalid memory reference
        // let str = get_str_at(1000, 10);
        // println!("str: {:?}", str);
    }

    #[test]
    pub fn test_create_raw_ptr_with_smart_ptr() {
        let ten = Box::new(10);
        let ptr1 = &*ten as *const i32;
        let ptr2: *const i32 = Box::into_raw(ten);
        println!("ptr1: {:?}, ptr2: {:?}", ptr1, ptr2);
    }

    unsafe fn dangerous() {
        println!("this is a dangerous function!!!");
    }
    #[test]
    pub fn test_call_unsafe_fun() {
        unsafe {
            dangerous();
        }
    }

    ///
    pub fn split_slice(slice: &mut [i32], position: usize) -> (&[i32], &[i32]) {
        assert!(position <= slice.len());

        // Runtime error: second mutable borrow occurs here
        // (&mut slice[..position], &mut slice[position..])

        let ptr = slice.as_ptr() as *const i32;
        unsafe {
            (
                slice::from_raw_parts(ptr, position),
                slice::from_raw_parts(ptr.add(position), slice.len() - position)
            )
        }
    }

    #[test]
    pub fn test_split_slice() {
        let mut arr = [1, 2, 3, 4, 5, 6];

        let arr_slice = &mut arr[..];
        let position = 3;

        let splitted = split_slice(arr_slice, position);
        println!("slice1: {:?}, slice2: {:?}", splitted.0, splitted.1);
    }

    /// FFI: foreign function interface
    extern "C" {
        // ABI: Application Binary Interface
        fn abs(arg: i32) -> i32;
    }
    #[test]
    pub fn test_call_ffi(){
        unsafe {
            println!("Absolute value of -3 is: {:?}", abs(-3));
        }
    }

    /// #[no_mangle] tells rustc, don't change function name
    #[no_mangle]
    pub extern "C" fn call_from_clang(){
        println!("Called a Rust function from clang");
    }
}

/// No `main` function found in crate `unsafes` [EO601]
fn main() {}