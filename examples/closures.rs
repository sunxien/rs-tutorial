#[derive(Debug)]
#[allow(dead_code, unused)]
struct Employee<'a> {
    id: u32,
    name: &'a mut String,
}

/// Closure on struct
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cacher<T> {
        Cacher { query, value: None }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                v
            }
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


// ///
// pub trait Operation<K, V> {
//     fn get(&self, key: K) -> V;
//     fn set(&self, key: K, value: V);
//     fn size(&self) -> usize;
// }
// #[derive(Debug)]
// struct Cache<'a, K, V> {
//     capacity: usize,
//     inner_cache: &'a mut HashMap<K, V>,
// }
// impl<'a, K, V> Cache<'a, K, V> {
//     fn new(capacity: usize) -> Self {
//         Cache { capacity, inner_cache: &mut HashMap::with_capacity(capacity) }
//     }
// }
// impl<'a, K, V> Operation<K, V> for Cache<'a, K, V> {
//     fn get(&self, key: K) -> V {
//         self.inner_cache.get(key)
//     }
//     fn set(&self, key: K, value: V) {
//         if self.size() > self.capacity {
//             panic!("Cache is overflow!!!")
//         }
//         self.inner_cache.insert(key, value);
//     }
//     fn size(&self) -> usize {
//         self.inner_cache.capacity()
//     }
// }

/// Rust Doc: https://course.rs/advance/functional-programing/closure.html
#[cfg(test)]
#[allow(dead_code, unused)]
pub mod closures_test_cases {
    use std::thread;
    use crate::{Cacher, Employee};

    /// closure is an anonymous function. But it's type fn(?) -> ?
    /// 1. assign `closure` to a variable.
    /// 2. capture the `variables` in the same scope. But it will allocate mem in the closure to store variable!!!!
    #[test]
    fn test_simple_closure() {
        let y = 32i32;
        let sum = |x| x + y; // allocate mem in the closure to store `y` variable!!!
        println!("Result: {}", sum(15));
    }

    /// 1. specify an explicit type `Employee`
    /// 2. specify a lifetime for closure.
    /// Rust Doc: https://course.rs/advance/lifetime/advance.html#%E9%97%AD%E5%8C%85%E5%87%BD%E6%95%B0%E7%9A%84%E6%B6%88%E9%99%A4%E8%A7%84%E5%88%99
    #[test]
    fn test_closure_for_explicit_type() {
        // Compile error: returning this value requires that `'1` must outlive `'2`
        // How to resolve lifetime for closure? 1. Not use ref; 2. Use fn trait; 3. Don't use closure;
        // let c = |mut e: Employee| -> Employee {
        //     let mut temp = e.name;
        //     temp.push_str("@alibaba-inc.com");
        //     e.name = temp;
        //     e
        // };
        //
        // let mut emp = Employee { id: 121001, name: &mut String::from("xien.sxe") };
        // println!("Employee: {:?}", c(emp));

        // Solution 1: Not use ref;
        // let c = |mut e: Employee| -> Employee {
        //     let mut temp = e.name;
        //     temp.push_str("@alibaba-inc.com");
        //     e.name = temp;
        //     e
        // };
        //
        // let mut emp = Employee { id: 121001, name: String::from("xien.sxe") };
        // println!("Employee: {:?}", c(emp));

        // Solution 2: Use fn trait;
        let cl = closure_lifetime1(|e: &mut Employee| -> &Employee {
            e.name.push_str("@alibaba-inc.com");
            e
        });
        let mut emp = Employee { id: 121001, name: &mut String::from("xien.sxe") };
        println!("Employee: {:?}", cl(&mut emp));
    }
    fn closure_lifetime1<T, C: Fn(&mut T) -> &T>(c: C) -> C
    {
        c
    }

    ///
    #[test]
    fn test_closure_for_lifetime() {
        // Compile error: returning this value requires that `'1` must outlive `'2`
        // let closure_expr = |x: &i32| -> &i32 { x };
        // let x = closure_expr(&4);

        let x = 4;
        let closure_slision = closure_lifetime2(|x: &i32| -> &i32 { x });
        println!("x: {:?}, y: {:?}", x, closure_slision(&4));
    }
    fn closure_lifetime2<T, C: Fn(&T) -> &T>(c: C) -> C
    {
        c
    }

    ///
    #[test]
    #[allow(dead_code, unused)]
    fn test_closure_for_struct() {
        let mut c = Cacher::new(|x| x);
        let v1 = c.value(1);
        let v2 = c.value(1);
        let v3 = c.value(2);
        // TODO How does it mean????
    }

    /// move ownership, run more times
    fn test_fn_closure() {}

    /// mutable borrow, run more times
    #[test]
    fn test_fnmut_closure() {
        let mut empty = String::new();

        // Compile error: calling `closure_expr` requires mutable binding due to mutable borrow of `empty`
        let mut closure_expr1 = |arg| empty.push_str(arg);

        // Solution 1: let mut closure = ....
        closure_expr1("hello closure_expr1");
        println!("[test_fnmut_closure] closure_expr1 result: {:?}", empty);

        // Compile error: `impl Trait` is not allowed in the type of variable bindings
        // let closure_expr2: impl FnMut(&str) = |arg| empty.push_str(arg);
        // closure_expr2("hello closure_expr2");
        fnmut_update(|arg| empty.push_str(arg), " hello closure_expr2");
        println!("[test_fnmut_closure] closure_expr2 result: {:?}", empty);
    }
    /// Input `c:C` has moved into function, and it has not impl `Copy` trait,
    /// why it runs more than one times? Rust Doc: https://course.rs/advance/functional-programing/closure.html#move-%E5%92%8C-fn
    fn fnmut_update<C: FnMut(&str)>(mut c: C, arg: &str) {
        c(arg);
        c(arg);
    }


    /// immutable borrow, run only once
    fn check_size<C>(c: C)
    where
        C: FnOnce(usize) -> bool + Copy,
    {
        println!("[test_fnonce_closure] check_size 1 time: {:?}", c(256));
        // Compile error: `c` moved due to this call before!!!
        // Solution: add `Copy` trait for closure
        println!("[test_fnonce_closure] check_size 2 time: {:?}", c(1024));
    }
    #[test]
    fn test_fnonce_closure() {
        check_size(|x: usize| x > 512);

        let vecs = vec![1, 3, 4, 5, 7, 9];
        check_size(|x: usize| x > vecs.len());
    }

    /// If closure's lifetime is greater than external variable's lifetime, please use `move` keyword
    #[test]
    fn test_move_for_closure() {
        let vecs = vec![1, 2, 3, 4, 5];
        //Compile error: may outlive borrowed value `vecs`, so move ownership into thread.
        let t = thread::spawn(move || {
            println!("[test_move_for_closure] in thread: {:?}", vecs);
        });
        t.join().unwrap();
        // Compile error: borrow of moved value: `vecs`
        // println!("[test_move_for_closure] out thread: {:?}", vecs);
    }
}

fn main() {}