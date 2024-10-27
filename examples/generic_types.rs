use std::fmt::Debug;

/// Generic Type define in `enum`
enum Error<I, E> {
    SYSTEM_ERROR(I, E)
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
}

/// No `main` function found in crate `generic_types` [EO601]
fn main() {}