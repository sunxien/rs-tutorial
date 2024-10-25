struct Wrapper<T> {
    value: T,
}

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

#[cfg(test)]
#[allow(unused, dead_code)]
pub mod generic_types_test_cases {
    use crate::Wrapper;

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

    }
}

/// No `main` function found in crate `generic_types` [EO601]
fn main() {}