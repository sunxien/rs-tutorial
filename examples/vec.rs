#[cfg(test)]
mod vec_test_cases {

    #[test]
    fn test_vec1() {
        let v = vec![1, 2, 3];
        for i in v {
            println!("value: {:?}", i);
        }
    }

    #[test]
    fn test_vec2() {
        let v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        for i in v {
            println!("value: {:?}", i);
        }
    }
}