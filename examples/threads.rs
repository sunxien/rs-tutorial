#[cfg(test)]
#[allow(dead_code, unused)]
pub mod threads_test_cases {
    use std::cell::RefCell;
    use std::ops::Add;
    use std::rc::Rc;
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::thread::{Builder, JoinHandle};
    use std::time::Duration;

    /// 1. thread::spawn
    /// 2. join().unwrap()
    /// Rust Doc: https://course.rs/advance/concurrency-with-threads/thread.html
    #[test]
    pub fn test_thread_spawn() {
        let subthread = thread::spawn(|| {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(1));
                println!("No.{:?} [SubThread] sleep 1s....", i);
            }
        });
        // subthread.join().unwrap();

        for i in 1..=5 {
            thread::sleep(Duration::from_secs(1));
            println!("No.{:?} [MainThread] sleep 1s....", i);
        }
    }

    ///
    #[test]
    pub fn test_move_var() {
        let v = vec![1, 2, 3];
        // Runtime error: closure may outlive the current function, but it borrows `v`, which is owned by the current function
        let subthread = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(1));
                println!("No.{:?} [SubThread] sleep 1s and print: {:?}....", i, v);
            }
        });
        // main thread block, wait sub thread run finished.
        subthread.join().unwrap();

        // for i in 1..=5 {
        //     thread::sleep(Duration::from_secs(1));
        //     // Runtime error: error[E0382]: borrow of moved value: `v`
        //     println!("No.{:?} [MainThread] sleep 1s and print: {:?}....", i, v);
        // }
    }

    //
    #[test]
    pub fn test_barrier() {
        let mut threads: Vec<JoinHandle<()>> = vec![];
        let barrier = Arc::new(Barrier::new(3));
        let prefix = String::from("thread-");
        for i in 1..=3 {
            let mut name = String::from("thread-");
            name.push_str(i.to_string().as_str());
            let cloned_barrier = barrier.clone();
            let builder = Builder::new();
            let t = builder.name(name).spawn(move || {
                println!("Current thread: {:?} is begin....", thread::current().name());
                cloned_barrier.wait();
                println!("Current thread: {:?} is finished!", thread::current().name());
            }).unwrap();
            threads.push(t);
        }
        for e in threads {
            e.join().unwrap();
        }
    }

    ///
    #[test]
    pub fn test_thread_local() {}

    ///
    #[test]
    pub fn test_condvar_mutex() {}

    ///
    #[test]
    pub fn test_mpsc() {}

    ///
    #[test]
    pub fn test_lock() {}

    ///
    #[test]
    pub fn test_semaphore() {}

    ///
    #[test]
    pub fn test_atomic() {}

    ///
    #[test]
    pub fn test_sync_send() {}

    /// Other concurrent problems.
    #[test]
    pub fn test_multi_threads_add() {
        
    }
}

/// No `main` function found in crate `threads` [EO601]
fn main() {}