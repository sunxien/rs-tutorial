#[cfg(test)]
#[allow(dead_code, unused)]
pub mod async_await_test_cases {
    use std::rc::Rc;
    use std::thread::sleep;
    use std::time::Duration;

    use async_std::task;
    use futures::executor::block_on;

    ///
    async fn test_print_with_future() {
        println!("Go Go Go!!!");
    }

    /// async-std, tokio
    /// tonic, actix, graphQL
    #[test]
    pub fn test_async() {
        let f = test_print_with_future();
        println!("invoke `test_print_with_future()` done!");
        block_on(f);
        println!("await `test_print_with_future()` return!");
    }

    ///
    async fn test_hello_rust() {
        test_hello_world().await; // if not `await`, or not block_on(...), ignore call this method
        println!("hello rust");
    }

    async fn test_hello_world() {
        println!("hello world")
    }

    /// The difference between `await` and `block_on(f)`:
    /// not block current thread.
    #[test]
    pub fn test_async_call_async() {
        let f = test_hello_rust();
        println!("invoke `test_hello_rust()` done!");
        block_on(f);
    }

    ///
    async fn learn() {
        println!("learn now....");
        // different between `thread::sleep` and `task::sleep`?
        // thread::sleep(Duration::from_secs(2));
        task::sleep(Duration::from_secs(2)).await;
    }
    async fn sing() {
        println!("sing now....");
        // different between `thread::sleep` and `task::sleep`?
        // thread::sleep(Duration::from_secs(2));
        task::sleep(Duration::from_secs(2)).await;
    }
    async fn learn_and_sing() {
        learn().await;
        sing().await;
    }
    async fn dance() {
        println!("dancing now....");
    }
    async fn call_async_methods() {
        let f1 = learn_and_sing();
        let f2 = dance();
        futures::join!(f1, f2);
    }
    #[test]
    pub fn test_call_async_methods() {
        block_on(call_async_methods());
    }

    /// Smart-Pointer, Weak Ref, Self Ref,
    #[test]
    pub fn test_weak_ref() {
        let a = Rc::new(5);
        println!("[test_weak_ref] ref of a: {:?}", Rc::strong_count(&a));

        let weak_a = Rc::downgrade(&a);
        println!("[test_weak_ref] weak_a_weak_count: {:?}, weak_a_strong_count: {:?}", weak_a.weak_count(), weak_a.strong_count());

        let weak_a_opt = weak_a.upgrade();
        println!("[test_weak_ref] before drop weak_a_opt: {:?}", weak_a_opt.unwrap());

        std::mem::drop(a);

        let weak_a_opt = weak_a.upgrade();
        println!("[test_weak_ref] after drop weak_a_opt: {:?}", weak_a_opt);
    }


    /// Rust Doc: https://course.rs/advance/circle-self-ref/circle-reference.html
    #[test]
    pub fn test_cycle_reference() {
        // TODO
    }

    /// Rust Doc: https://course.rs/advance/circle-self-ref/self-referential.html
    #[derive(Debug)]
    struct SelfRef<'a> {
        val: String,
        ptr: &'a str, // point to value
    }
    #[derive(Debug)]
    struct SelfRefOpt<'a> {
        val: String,
        ptr: Option<&'a str>,
    }
    #[test]
    pub fn test_self_reference() {
        let val = String::from("hello");
        // Compile error: Value used after being moved [E0382]
        // let self_ref = SelfRef { val, ptr: val.as_str() };

        // Use `Option`
        let mut self_ref_opt = SelfRefOpt { val:"Hello".to_string(), ptr:Option::None };
        self_ref_opt.ptr = Some(&self_ref_opt.val[..]);
        println!("[test_self_reference] {:?}", self_ref_opt);
    }
}

/// async/await: https://course.rs/advance/async/getting-started.html
fn main() {}