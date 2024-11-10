#[cfg(test)]
#[allow(dead_code, unused)]
pub mod async_await_test_cases {
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
}

/// async/await: https://course.rs/advance/async/getting-started.html
fn main() {}