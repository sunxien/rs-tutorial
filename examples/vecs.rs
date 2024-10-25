#[cfg(test)]
mod vecs_test_cases {

    #[test]
    fn test_vec1() {
        let v = vec![1, 2, 3];
        for i in v {
            println!("[test_vec1] value: {:?}", i);
        }
    }

    #[test]
    fn test_vec2() {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        for i in v {
            println!("[test_vec2] value: {:?}", i);
        }
    }

    #[test]
    fn test_vec3() {
        let mut v = Vec::with_capacity(1);
        v.push(1);
        v.push(2);
        v.push(3);
        for i in v {
            println!("[test_vec3] value: {:?}", i);
        }
    }
}

/// No `main` function found in crate `vecs` [EO601]
fn main() {}