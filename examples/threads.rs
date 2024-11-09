#[cfg(test)]
#[allow(dead_code, unused)]
pub mod threads_test_cases {
    use std::ops::Sub;
    use std::sync::{Arc, Barrier, Condvar, mpsc, Mutex, RwLock};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::thread;
    use std::thread::{Builder, JoinHandle, sleep};
    use std::time::Duration;

    use lazy_static::lazy_static;
    use tokio::time::Instant;

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
            let t = Builder::new().name(name).spawn(move || {
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

    /// single producer, single consumer
    #[test]
    pub fn test_spsc() {
        let (p, c) = mpsc::channel();
        let t = thread::spawn(move || {
            p.send(1).unwrap();
            // Compile error: expected `i32`, but found `Option<i32>`
            // p.send(Some(1)).unwrap();
            println!("[test_mpsc] produce a message success");
        });
        // t.join().unwrap();
        match c.recv() {
            Ok(v) => {
                println!("[test_mpsc] receive a i32: {:?}", v);
            }
            Err(e) => {
                eprintln!("[test_mpsc] receive an error. Kind: {:?}", e);
            }
        }
    }

    /// multiple producer, single consumer
    #[test]
    pub fn test_mpsc() {
        let parallism = 3;
        let (prod, cons) = mpsc::channel();
        for i in 1..=parallism {
            let cloned_prod = prod.clone();
            let t = thread::spawn(move || {
                // compile error: Value used after being moved [E0382]
                // prod.send(i).unwrap();
                cloned_prod.send(i).unwrap();
            });
        }
        // all producer should be dropped!!!
        drop(prod);
        for message in cons {
            println!("[test_mpsc] receive a message: {:?}", message);
        }
        println!("[test_mpsc] all messages are received!");
    }

    /// multiple producer, multiple consumer
    /// third-party lib: crossbeam-channel, flume
    #[test]
    pub fn test_mpmc() {
        // TODO code snippet
    }

    /// Mutex is a smart-pointer
    /// Rust Doc: https://course.rs/advance/concurrency-with-threads/sync1.html
    #[test]
    pub fn test_mutex() {
        let mutex = Mutex::new(5);
        {
            let mut val = mutex.lock().unwrap();
            *val = *val + 1;
        } // Mutex will be dropped here
        println!("[test_mutex] value: {:?}", mutex);
    }

    /// Rc is not implement `Send` trait
    /// Rc + RefCell is used for single thread
    /// Arc + Mutex is used for multiple thread
    /// Rust Doc: https://course.rs/advance/concurrency-with-threads/sync1.html
    #[test]
    pub fn test_mutex_with_multi_threads() {
        let parallel = 5;
        let arc_barrier = Arc::new(Barrier::new(parallel));
        let mut threads = Vec::with_capacity(parallel);

        let arc_mutex = Arc::new(Mutex::new(0));
        for i in 1..=parallel {
            let mut tname = String::from("thread-");
            tname.push_str(i.to_string().as_str());
            let cloned_bar = Arc::clone(&arc_barrier);
            let cloned_arc = Arc::clone(&arc_mutex);
            threads.push(Builder::new().name(tname).spawn(move || {
                println!("[test_mutex_with_multi_threads] thread: {:?} is ready....", thread::current().name());
                cloned_bar.wait();
                let mut val = cloned_arc.lock().unwrap();
                *val = *val + 1;
                println!("[test_mutex_with_multi_threads] thread: {:?}, val: {:?}", thread::current().name(), val);
            }).unwrap());
        }
        for thread in threads {
            thread.join().unwrap();
        }
        println!("[test_mutex_with_multi_threads] value: {:?}", arc_mutex);
    }

    ///
    lazy_static! {
        static ref M1: Mutex<i32> = Mutex::new(1);
        static ref M2: Mutex<i32> = Mutex::new(1);
    }
    /// This will occur a dead lock.
    // #[test]
    pub fn test_dead_lock() {
        let parallel = 2;
        let arc_barrier = Arc::new(Barrier::new(parallel));
        let mut threads = Vec::new();
        for i in 1..=parallel {
            let cloned_barrier = arc_barrier.clone();
            threads.push(thread::spawn(move || {
                // cloned_barrier.wait();
                if i % 2 == 0 {
                    let guard1 = M1.lock().unwrap();
                    println!("[test_dead_lock] acquire M1 lock success, and try to acquire M2 lock....");
                    sleep(Duration::from_millis(200));
                    let guard2 = M2.lock().unwrap();
                } else {
                    let guard2 = M2.lock().unwrap();
                    println!("[test_dead_lock] acquire M2 lock success, and try to acquire M1 lock....");
                    sleep(Duration::from_millis(200));
                    let guard1 = M1.lock().unwrap();
                }
            }));
        }
        for thread in threads {
            thread.join().unwrap();
        }
        println!("[test_dead_lock] dead lock not found!");
    }

    ///
    #[test]
    pub fn test_rwlock() {
        let rwlock = RwLock::new(5);
        // read lock can be acquired several times.
        {
            let rlock1 = rwlock.read().unwrap();
            let rlock2 = rwlock.read().unwrap();
            println!("[test_rwlock] read lock1: {:?}, read lock2: {:?}", *rlock1, *rlock2);
        } // read lock is dropped here
        // write lock is conflict with any other locks (read or write).
        {
            let mut wlock = rwlock.write().unwrap();
            *wlock = *wlock + 1;
            println!("[test_rwlock] write lock: {:?}, and it will BLOCK when acquire a read lock", *wlock);
            // rwlock.read().unwrap();
        } // write lock is dropped here
    }

    ///

    pub trait IPrint {
        fn print(&mut self);
    }
    struct Printer {
        id: usize, // from 0
        name: String,
        chars: Arc<Vec<char>>,
        mutex: Arc<Mutex<usize>>,
        cond: Arc<Condvar>,
    }
    impl Printer {
        fn new(id: usize, name: String, mutex: Arc<Mutex<usize>>, cond: Arc<Condvar>, chars: Arc<Vec<char>>) -> Self {
            Printer {
                id,
                name,
                mutex,
                cond,
                chars,
            }
        }
    }
    impl IPrint for Printer {
        fn print(&mut self) {
            loop {
                let mut signal = self.mutex.lock().unwrap();
                while *signal != self.id {
                    signal = self.cond.wait(signal).unwrap();
                }
                let symbol: &char = self.chars.get(self.id).unwrap();
                // cargo test --nocapture to resolve println! is not visible in unit-test
                println!("[test_condvar] {:?} is printing {:?}", self.name, symbol);
                *signal = (*signal + 1) % 4;
                sleep(Duration::from_millis(300));
                self.cond.notify_all();
            } // lock should be dropped here
        }
    }
    #[test]
    pub fn test_condvar() {
        let arc_chars = Arc::new(vec!['A', 'B', 'C', 'D']);
        let arc_mutex = Arc::new(Mutex::new(0));
        let arc_condvar = Arc::new(Condvar::new());
        let mut printers = Vec::new();
        for i in 0..4 {
            let cloned_arc_mutex = Arc::clone(&arc_mutex);
            let cloned_arc_condvar = Arc::clone(&arc_condvar);
            let cloned_arc_chars = Arc::clone(&arc_chars);

            let mut thread_name = String::from("Printer-");
            thread_name.push_str(cloned_arc_chars[i].to_string().as_str());

            let mut printer = Printer::new(i, thread_name, cloned_arc_mutex, cloned_arc_condvar, cloned_arc_chars);
            printers.push(Builder::new().spawn(move || {
                printer.print();
            }).unwrap());
        }

        for printer in printers {
            printer.join().unwrap();
        }
        // TODO Another question is How to implement a `producer-consumer` model?
    }

    /// Runtime error: error: async functions cannot be used for tests
    // #[test]
    pub fn test_semaphore() {

        // let parallel = 5;
        // let limit = 5;
        // let arc_semaphore = Arc::new(Semaphore::new(limit));
        //
        // let mut threads = Vec::with_capacity(parallel);
        //
        // for _ in 1..=5 {
        //     // compile error: `await` is only allowed inside `async` functions and blocks
        //     let permit = arc_semaphore.clone().acquire_owned().await.unwrap();
        //     threads.push(thread::spawn(move || {
        //         let datetime = Utc::now().format("yyyy-MM-dd HH:mm:ss");
        //         println!("[test_semaphore] {:?} thread run concurrently limit by {:?}", datetime, limit);
        //         sleep(Duration::from_secs(1));
        //         drop(permit); // release semaphore
        //     }));
        // }
        // for thread in threads {
        //     thread.join().unwrap();
        // }
    }

    /// Atomic is always used as Global Variable
    /// Rust Doc: https://course.rs/advance/concurrency-with-threads/sync2.html
    const N_TIMES: u64 = 10000000; // declare const must with type
    const N_THREADS: u8 = 8;
    static R: AtomicU64 = AtomicU64::new(0);
    // What's the difference between `const` and `static`?
    fn add_n_times(n: u64) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 1..=n {
                R.fetch_add(1, Ordering::Relaxed);
            }
        })
    }
    #[test]
    pub fn test_atomic() {
        let begin = Instant::now();
        let mut handlers = Vec::new();
        for _ in 1..=N_THREADS {
            handlers.push(add_n_times(N_TIMES));
        }
        for handle in handlers {
            handle.join().unwrap();
        }
        let result = R.load(Ordering::Relaxed);
        let elapsed = Instant::now().sub(begin);
        println!("[test_atomic] final result: {:?}. Elapsed: {:?}", result, elapsed);
    }

    ///
    #[test]
    pub fn test_sync_send() {}

    /// Other concurrent problems.
    #[test]
    pub fn test_multi_threads_add() {}
}

/// No `main` function found in crate `threads` [EO601]
fn main() {}