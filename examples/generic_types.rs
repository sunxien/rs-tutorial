use std::fmt::Debug;

/// Generic Type define in `enum`
enum Error<I, E> {
    SYSTEM_ERROR(I, E)
}

/// Generic Type define in `method` or `function`
/// Best Practice: return reference of T
fn largest<T: PartialOrd + Copy>(arr: &[T]) -> T {
    // As T is not impl Copy trait, so it occurs error
    // cannot move out of type [T], a non-copy slice
    let mut max = arr[0];
    for &value in arr.iter() {
        if max < value {
            max = value;
        }
    }
    max
}

/// Generic Type define in `method` or `function`
fn max_value<T: PartialOrd>(arr: &[T]) -> &T {
    let mut max_value = &arr[0];
    for value in arr.iter() {
        if max_value < value {
            max_value = value;
        }
    }
    &max_value
}

/// Generic Type define in `struct`
struct Wrapper<T> {
    value: T,
}

/// Notice: impl<T> is declared
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

impl Wrapper<isize> {
    pub fn abs(self) -> Wrapper<usize> {
        Wrapper { value: self.value.abs() as usize }
    }
}

/// const Generic Type (since Rust 1.51)
// `const N: usize` is a generic type limit for array. (Since Rust 1.51)
fn print_array<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("{:?}", arr);
}

/// const function
const fn add(a: usize, b: usize) -> usize {
    a + b
}
const RESULT: usize = add(5, 10);

/// Rust Doc: https://course.rs/basic/trait/generic.html#%E6%B3%9B%E5%9E%8B-generics
/// struct, enum, func, const
#[cfg(test)]
#[allow(unused, dead_code)]
pub mod generic_types_test_cases {
    use crate::{max_value, Wrapper};

    ///
    #[test]
    pub fn test_generic_types1() {
        let wrapper = Wrapper::new(-121001isize);
        assert_eq!(121001usize, wrapper.abs().value);

        let wrapper = Wrapper::new("121001");
        // wrapper.abs(); // abs() is not visible for &str type
    }

    /// generic types bound
    #[test]
    pub fn test_generic_types2() {
        let arr = [1, 2, 3, 4, 5];
        let max = max_value(&arr);
        println!("{:?}", max);
    }

    /// 1. generic type: func, struct, enum
    /// 2. generic type: const N, const expr, const fn
    /// 3. generic type: as parameter:
    /// Compile error: doesn't have a size known at compile-time
    fn length<T: Sized, const N: usize>(arr: [T; N]) -> usize {
        N
        // `T: Sized`, it can't be defined as `T:?Sized`
        // Compile error: doesn't have a size known at compile-time
    }
    #[derive(Debug)]
    struct Buffer<T, const N: usize> {
        data: [T; N],
    }
    const fn static_length_of(factor: usize) -> usize {
        factor * 4
    }
    #[test]
    pub fn test_dyn_arr_length() {
        let arr: [i32; 3] = [12, 33, 21];
        println!("arr: {}", length(arr));

        //...
        const SIZE: usize = static_length_of(2);
        let buf = Buffer::<i32, SIZE> { data: [-1; SIZE] };
        println!("const fn + const generic type: {:?}", buf);
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/// No `main` function found in crate `generic_types` [EO601]
fn main() {
    let p = Point { x: Point { x: 5, y: 10 }, y: Point { x: 5, y: 10 } };
    println!("p.x = {:?}", p.x());
}