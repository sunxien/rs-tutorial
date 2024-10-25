#[cfg(test)]
pub mod conditions_test_cases {
    use rand::random;

    /// if
    #[test]
    pub fn test_if() {
        for i in 1..11 {
            /// Rust Doc: https://docs.rs/rand/latest/rand/
            let num: i32 = rand::random();
            if num & 1 == 0 {
                println!("[test_if] No.{:?} It's an ODD number. ({:?})", i, num);
            } else {
                println!("[test_if] No.{:?} It's an EVEN number. ({:?})", i, num);
            }
        }
    }


    /// for
    /// Rust Docs: https://course.rs/basic/flow-control.html
    #[test]
    pub fn test_for() {
        for i in 1..=10 {
            if i == 4 {
                continue;
            }
            if i > 8 {
                break;
            }
            println!("[test_for] value: {:?}", i);
        }

        let vec = vec![1, 2, 3, 4, 5];
        /// ownership is moving into the `for` loop. use `&vec` instead
        /// Same as `for v in IntoIterator::into_iter(vec)`
        for v in &vec {
            println!("[test_for] value: {:?}", v);
        }
        println!("[test_for] value: {:?}", vec.len()); // compile error if not use `&vec`

        for (i, v) in vec.iter().enumerate() {
            println!("[test_for] vec[{:?}]: {:?}", i, v);
        }
        println!("[test_for] value: {:?}", vec.len()); // compile Passed!!!
    }

    /// loop
    /// `loop` is an expression, so it can return a result.
    #[test]
    pub fn test_loop() {
        let mut n = 5;
        loop {
            if n < 0 {
                break;
            }
            println!("[test_loop] run {:?} times....", n);
            n = n - 1;
        }
        println!("[test_loop] run finished!!!");

        let mut m = 0;
        let result = loop {
            m = m + 1;
            if (m > 10) {
                break m; // `break` can return a result
            }
        };
        println!("[test_loop] final result is {:?}", result);
    }

    /// while
    #[test]
    pub fn test_while() {
        let mut n = 5;
        while n > -1 {
            println!("[test_while] run {:?} times....", n);
            n = n - 1;
        }
        println!("[test_while] run finished!!!")
    }

    /// match, matches!()
    /// Rust Doc: https://course.rs/basic/match-pattern/match-if-let.html
    #[test]
    pub fn test_match() {
        enum Color {
            StrDesc(String), // "green"
            RgbCode(u16, u16, u16), // 248,248,255
            HexCode(String), // "#000000"
        }
        let colors = [
            Color::StrDesc(String::from("green")),
            Color::RgbCode(248, 248, 255),
            Color::HexCode(String::from("#000000"))
        ];
        let len = colors.len();
        for i in 1..=5 {
            let rand: usize = random();
            let color = match &colors[rand % len] {
                Color::StrDesc(desc) => "desc",
                Color::RgbCode(r, g, b) => "rgb",
                Color::HexCode(code) => "hex",
                _ => "unknown"
            };
            println!("Congratulations!!! Color indicates by: {:?}", color);
        }
    }

    /// if let
    /// Same as MATCH pattern { ... }
    /// See also to `if-let chains` with eRFC-2947
    #[test]
    pub fn test_if_let() {
        let tup = ("Pony", "Jack");
        if let ("Pony", v) = tup {
            println!("[test_if_let] if let v: {:?}", v);
        } else if let ("Jack", v) = tup {
            println!("[test_if_let] else if let v: {:?}", v);
        } else {
            println!("[test_if_let] else missing branch");
        }

        enum E {
            X(u8),
            Y(u8),
            Z(u8),
        }
        let v = E::X(12);
        if let E::X(n) | E::Y(n) = v {
            assert_eq!(n, 12);
        }
    }
}

/// No `main` function found in crate `conditions` [EO601]
fn main() {}