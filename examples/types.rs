use std::fmt::{Display, Formatter};
use std::ops::Add;

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

///
struct Meters(f32);
impl Copy for Meters {}
impl Clone for Meters {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl Display for Meters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}米", self.0)
    }
}
impl Add for Meters {
    // TODO What does `Output` mean?
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

///


/// Rust Doc: https://course.rs/advance/into-types/custom-type.html
#[allow(dead_code, unused)]
#[cfg(test)]
pub mod type_test_cases {
    use num_derive::FromPrimitive;

    use crate::{Meters, Wrapper};

    ///
    #[test]
    pub fn test_newtype() {
        let w = Wrapper(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        println!("[test_newtype] wrapper value: {}", w);

        let m1 = Meters(3.75f32);
        let m2 = Meters(3.5f32);
        let sum = m1 + m2;
        println!("[test_newtype] {} + {} = {}", m1, m2, sum);
    }

    /// type alias
    type Second = u32;
    #[test]
    pub fn test_type_alias() {
        let a: u32 = 1024;
        let b: Second = 1024;
        println!("[test_type_alias] {} + {} = {} ", a, b, (a + b));
    }

    ///
    type Thunk = Box<dyn Fn() + Send + 'static>;
    fn return_long_type_simple() -> Thunk {
        Box::new(|| println!("hi"))
    }
    fn call_long_type_simple(f: Thunk) {
        f()
    }
    #[test]
    pub fn test_simple_type() {
        let f: Box<dyn Fn() + Send + 'static> = return_long_type();
        call_long_type(f);

        // Simplify the
        let f: Thunk = return_long_type_simple();
        call_long_type_simple(f);
    }
    fn return_long_type() -> Box<dyn Fn() + Send + 'static> {
        Box::new(|| println!("hi"))
    }
    fn call_long_type(f: Box<dyn Fn() + Send + 'static>) {
        f()
    }

    /// never return type: `!`
    pub fn test_never_return_type() {
        let i = 2;
        let v = match i {
            1..3 => i,  // return integer type
            // _ => println!(".....") // return `()` type
            _ => panic!("......") // return `!` type
        };
    }

    /// DST, unsized type: array, slice, str, trait
    trait TraitObj {}
    // fn foobar1(p: TraitObj) {} // Compile error: Trait `std::marker::Sized` is not implemented for `dyn TraitObj` [E0277]
    fn foobar2(p: &dyn TraitObj) {}
    fn foobar3(p: Box<dyn TraitObj>) {}
    fn foobar4<T: ?Sized>(t: &T) {} // accept dyn type or sized type

    /// TODO What is `into()`, `try_into()`?
    /// Rust Doc: https://course.rs/advance/into-types/converse.html
    #[test]
    pub fn test_str_with_box() {
        let x: Box<str> = "a".to_string().into_boxed_str();
        println!("[test_str_with_box] into_boxed_str: {}", x);

        // Compile error: Cast to unsized type: `&str` as `str` [E0620]
        // let y: Box<str> = Box::new("b" as str);
        // println!("[test_str_with_box] as str: {}", x);

        let z: Box<str> = "c".into();
        println!("[test_str_with_box] into: {}", z);

        let a: Box<str> = "a".try_into().unwrap();
        println!("[test_str_with_box] into: {}", a);
    }

    ///
    #[derive(Debug, FromPrimitive)]
    enum Letter {
        A = 1,
        B,
        C,
        D,
    }
    impl TryFrom<u8> for Letter {
        type Error = (Self);
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                x if x == Letter::A as u8 => Result::Ok(Letter::A),
                x if x == Letter::B as u8 => Result::Ok(Letter::B),
                x if x == Letter::C as u8 => Result::Ok(Letter::C),
                x if x == Letter::D as u8 => Result::Ok(Letter::D),
                _ => panic!("unknown value: {}", value)
            }
        }
    }
    #[test]
    pub fn test_enum_to_number() {
        let a = Letter::A as u8;
        println!("[test_enum_to_number] a: {}", a);
        let b = Letter::B as u8;
        println!("[test_enum_to_number] b: {}", b);
        let c = Letter::C as u8;
        println!("[test_enum_to_number] c: {}", c);
        let d = Letter::D as u8;
        println!("[test_enum_to_number] d: {}", d);

        /// Compile error: Type mismatch [E0308] expected `u8`, but found `Letter`
        // match d {
        /// Solution1: use third-party lib: `num-traits`, `num-derive`
        // match num_traits::FromPrimitive::from_u8(d) {
        /// Solution2: TryFrom + macro
        /// match d.try_into().unwrap() {
        /// Solution3: use custom define macro.
        /// TODO Rust Doc: https://course.rs/advance/into-types/enum-int.html
        /// Solution4: use `std::mem::transmute()`
        match transmute_unsafe(d) {
            Some(Letter::A) => println!("[test_enum_to_number] d: is Letter::A"),
            Some(Letter::B) => println!("[test_enum_to_number] d: is Letter::B"),
            Some(Letter::C) => println!("[test_enum_to_number] d: is Letter::C"),
            Some(Letter::D) => println!("[test_enum_to_number] d: is Letter::D"),
            _ => println!("[test_enum_to_number] d: is Letter::UNKNOWN"),
        }
    }
    fn transmute_unsafe(val: u8) -> Option<Letter> {
        unsafe {
            // let l: Letter = std::mem::transmute(val);
            // Some(l)
            Some(std::mem::transmute(val))
        }
    }
}


fn main() {}