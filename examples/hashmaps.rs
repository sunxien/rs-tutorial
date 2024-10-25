#[cfg(test)]
pub mod hashmap_test_cases {

    use std::collections::HashMap;

    #[test]
    pub fn test_hashmap1() {
        let mut map = HashMap::new();
        map.insert("id", "121001");
        map.insert("name", "sunxien");

        for (k, v) in map {
            println!("[test_hashmap1] Key: {:?}, Value: {:?}", k, v);
        }
    }

    #[test]
    pub fn test_hashmap2() {
        let mut map = HashMap::with_capacity(1);
        map.insert("id", "121001");
        map.insert("name", "sunxien");

        for (k, v) in map {
            println!("[test_hashmap2] Key: {:?}, Value: {:?}", k, v);
        }
    }
}

/// No `main` function found in crate `hashmaps` [EO601]
fn main() {}
