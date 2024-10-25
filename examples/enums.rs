enum Message<'a> {
    CONNECT(&'a str),
    LOCATION(Point),
    TERMINATE(i8),
}


struct Point {
    x: u8,
    y: u8,
    z: u8,
}

impl Point {

    ///
    pub fn Point(x: u8, y: u8, z: u8) -> Point {
        Point { x: x, y: y, z: z }
    }
}

#[cfg(test)]
pub mod enums_test_cases {

    #[test]
    pub fn test_enums1(){

    }

}

/// No `main` function found in crate `enums` [EO601]
fn main() {}