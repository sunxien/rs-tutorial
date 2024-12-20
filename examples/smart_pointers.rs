/// Rust Doc: https://course.rs/advance/smart-pointer/box.html
/// small data: allocate on stack
/// medium data: allocate on stack is better, but read is same
/// huge data: allocate on heap
#[cfg(test)]
#[allow(dead_code, unused)]
pub mod smart_pointers_test_cases {
    use std::cell::{Cell, RefCell};
    use std::fmt::{Display, Formatter, Pointer};
    use std::ops::{Add, Deref};
    use std::rc::Rc;
    use std::sync::Arc;
    use std::thread;
    use std::thread::{Builder, JoinHandle, Thread};
    use num_traits::ToPrimitive;
    use rocket::yansi::Paint;

    /// stack size in main thread: 8MB, others 2MB
    #[test]
    pub fn test_box() {
        let a = Box::new(1);
        println!("[test_box] a: {}", a); // auto call deref

        // let b = a + 1; // compile error: Cannot add `i32` to `Box<i32>` [E0369]
        let b = *a + 1; // deref smart pointer manually
        println!("[test_box] b: {}", b);
    }

    /// test for ownership
    #[test]
    pub fn test_box_for_ownership() {
        let arr = [1000; 3];
        let copy = arr; // this is a deep copy from arr which is allocating on stack.
        println!("[test_box_for_ownership] arr: {:?}, addr: {:?}", arr, (arr.as_ptr() as usize));
        println!("[test_box_for_ownership] copy: {:?}, addr: {:?}", copy, (copy.as_ptr() as usize));

        let arr_box = Box::new([10000; 3]);
        let copy_box = arr_box; // this is a ref copy from arr_box, the data is on heap.
        // compile error: error[E0382]: borrow of moved value: `arr_box`
        // println!("[test_box_for_ownership] arr_box: {:?}, addr: {:?}", arr_box, (arr_box.as_ptr() as usize));
        println!("[test_box_for_ownership] copy_box: {:?}, addr: {:?}", copy_box, (copy_box.as_ptr() as usize));
    }

    /// test for data structure.
    #[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
    enum Node {
        // compile error: error[E0072]: recursive type `Node` has infinite size
        // NODE(i32, Node),
        NODE(i32, Box<Node>), // use Box<Node> to resolve
        NULL,
    }
    impl Display for Node {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut vec: Vec<String> = Vec::new();
            let mut temp = self;
            let text = loop {
                match temp {
                    Node::NODE(data, next) => {
                        vec.push(data.to_string());
                        temp = next;
                    }
                    Node::NULL => {
                        break vec.join(" -> ")
                    }
                    _ => panic!("unknown node: {:?}", temp)
                }
            };
            write!(f, "{}", text)
        }
    }
    #[test]
    pub fn test_for_linked_list() {
        let n1 = Node::NODE(1, Box::new(Node::NULL));
        let n2 = Node::NODE(2, Box::new(n1));
        let n3 = Node::NODE(3, Box::new(n2));
        println!("[test_for_linked_list] linkedlist: {}", n3);
    }

    /// for trait object
    pub trait Draw {
        fn draw(&self);
    }
    struct Select {}
    impl Draw for Select {
        fn draw(&self) {
            println!("[test_for_trait_object] drawing select ...")
        }
    }
    struct Button {}
    impl Draw for Button {
        fn draw(&self) {
            println!("[test_for_trait_object] drawing button ...")
        }
    }
    #[test]
    pub fn test_for_trait_object() {
        // compile error: error[E0308]: mismatched types
        // let draws: [dyn Draw; 2] = [Button {}, Select {}];
        let draws: [Box<dyn Draw>; 2] = [Box::new(Button {}), Box::new(Select {})];
        for draw in draws {
            draw.draw();
        }

        let draws: Vec<Box<dyn Draw>> = vec![Box::new(Button {}), Box::new(Select {})];
        for draw in draws {
            draw.draw();
        }
    }

    /// change Box<String> to &'static str
    /// Rust Doc: https://course.rs/advance/smart-pointer/box.html
    #[test]
    pub fn test_box_leak() {
        let s = Box::new("box leak".to_string());
        let box_leak = Box::leak(s.into_boxed_str());
        println!("[test_box_leak] leak into_boxed_str: {}", box_leak);
    }

    /// Rust Doc: https://course.rs/advance/smart-pointer/deref.html
    struct MyBox<T> (T);
    impl<T> MyBox<T> {
        pub fn new(value: T) -> Self {
            MyBox(value)
        }
    }
    impl<T> Pointer for MyBox<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:p}", self.deref())
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = (T);
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    #[test]
    pub fn test_deref() {
        let x = 123;
        let y = &x;
        // Compile error: `i32` does not implement `Pointer` (required by `{:p}`) [E0277]
        // println!("[test_deref] value:{}, ptr: {:p}", x, x);
        println!("[test_deref] value:{}, ptr: {:p}", y, y);

        let one = Box::new(1);
        let one_deref = *one + 1; // auto deref
        println!("[test_deref] value:{}, ptr: {:p}", one_deref, one);

        let mb = MyBox::new(13);
        let mb_deref = *mb + 21;
        println!("[test_deref] value:{}, ptr: {:p}", mb_deref, mb);
        // println!("[test_deref] ");
        // println!("[test_deref] ");
        // println!("[test_deref] ");
    }

    /// Drop
    /// Copy and Drop are conflict!!! Rust Doc: https://course.rs/advance/smart-pointer/drop.html
    struct Foo {}
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("[test_drop] dropping foo now....");
        }
    }
    #[test]
    pub fn test_drop() {
        let f = Foo {};
        // f.drop(); // compile error: Explicit calls to `drop` are forbidden. Use `std::mem::drop` instead. [E0040]
        std::mem::drop(f);
        // only move ownership, destructor finally end the scope
    }

    /// Cow: Copy on write
    #[test]
    pub fn test_cow() {}

    /// Rc: Reference count
    #[test]
    pub fn test_rc() {
        let s = String::from("hello world");
        let a = Box::new(s);
        // let b = Box::new(s); // compile error: Value used after being moved [E0382]

        let a = Rc::new(String::from("hello rust"));
        println!("[test_rc] strong count `a`: {}", Rc::strong_count(&a));
        let b = Rc::clone(&a); // this is shallow copy. same with `a.clone()`
        println!("[test_rc] strong count `a`: {}, `b`: {}", Rc::strong_count(&a), Rc::strong_count(&b));
        {
            let c = Rc::clone(&b);
            println!("[test_rc] strong count `a`: {}, `b`: {}, `c`: {}", Rc::strong_count(&a), Rc::strong_count(&b), Rc::strong_count(&c));
        }
        println!("[test_rc] strong count `a`: {}, `b`: {}", Rc::strong_count(&a), Rc::strong_count(&b));
    }

    /// Arc: Atomic reference count
    #[test]
    pub fn test_arc() {
        // let rc = Rc::new(String::from("hello rust"));
        // Compile error: F: Send + 'static is required by `move ||` closure!!!
        // let t = thread::spawn(move || {
        //     println!("Rc value: {:?}", rc);
        // });
        // t.join().unwrap();

        let arc = Arc::new(String::from("hello rust"));
        let t = thread::spawn(move || {
            println!("[test_arc] Rc value: {:?}", arc);
        });
        t.join().unwrap();
    }

    /// Cell: https://course.rs/advance/smart-pointer/cell-refcell.html
    /// `Cell` is used for value which implement by `T: Copy`
    #[test]
    pub fn test_cell() {
        let mut cell = Cell::new(1);
        let mut v = cell.take();
        v = v + 1;
        cell.set(v);
        println!("[test_cell] Cell value: {:?}", cell);
    }

    /// RefCell: https://course.rs/advance/smart-pointer/cell-refcell.html
    /// `RefCell` is used for reference which implement by `T: Send + 'static`
    #[test]
    pub fn test_ref_cell() {
        // let value = 1;
        // let mut refCell = RefCell::new(&value);
        // let mut threads = Vec::new();
        // for i in 1..=5 {
        //     let t = thread::spawn(|| {
        //         let mut v = refCell.borrow_mut();
        //         let r = v.add(1); // let
        //     });
        //     threads.push(t);
        // }
        // for t in threads {
        //     t.join().unwrap();
        // }
        // println!("[test_ref_cell] RefCell value: {:?}", refCell);
    }

    /// trait define external
    pub trait Messager {
        fn send(&self, message: String);
    }

    /// implement define internal
    pub struct MessageQueue {
        // buffer_cache: Vec<String>
        buffer_cache: RefCell<Vec<String>>,
    }

    impl MessageQueue {
        fn new(capacity: u32) -> Self {
            MessageQueue { buffer_cache: RefCell::new(Vec::new()) }
        }
    }

    impl Messager for MessageQueue {
        fn send(&self, message: String) {
            // Compile error: Cannot borrow immutable local variable `self.bufferCache` as mutable
            // self.bufferCache.push(message);

            // Runtime error: cannot borrow data in dereference of `std::cell::Ref<'_, Vec<std::string::String>>` as mutable
            // self.bufferCache.borrow().push(message);

            self.buffer_cache.borrow_mut().push(message);
        }
    }

    ///
    #[test]
    pub fn test_ref_cell_for_mq() {
        let mq = MessageQueue::new(4);
        mq.send("hello mq".to_string());
    }

    ///
    #[test]
    pub fn test_rc_with_ref_cell() {
        let rc = Rc::new(RefCell::new(1));
        let c1 = rc.clone();
        let c2 = rc.clone();
        let b = c2.borrow_mut().add(1025);
        println!("[test_rc_with_ref_cell] ref count: {:?}, ref count: {:?}, add: {:?}", //
                 Rc::strong_count(&c1), Rc::strong_count(&c2), b);
        // Rc::weak_count()
    }

    /// cycle reference
    #[test]
    pub fn test_cycle_reference() {}

    /// self reference
    struct SelfRef<'a> {
        value: String,
        ptr: &'a str,
    }
    #[test]
    pub fn test_self_reference() {
        // let s = String::from("hi");
        // let self_ref = SelfRef {
        //     value: s,
        //     ptr: &s, // compile error: Value used after being moved [E0382]
        // };
        // compile error: Value used after being moved [E0382]
        // Solution 1: Option
        // Solution 2: unsafe + raw pointer
        // Solution 3: Pin
    }
    #[test]
    pub fn test_self_reference_with_option() {}
    struct UnsafeSelfRef {
        value: String,
        ptr: *const String, // or change to *mut String
    }
    impl UnsafeSelfRef {
        pub fn new(val: &str) -> Self {
            UnsafeSelfRef {
                value: val.to_string(),
                ptr: std::ptr::null(), // *const T
            }
        }
        pub fn init(&mut self) {
            // what does *const String mean???
            // Rust Doc: https://doc.rust-lang.net.cn/stable/reference/types/pointer.html
            let self_ref: *const String = &self.value; // change to &mut self.value;
            self.ptr = self_ref;
        }
        pub fn value(&self) -> &str {
            &self.value
        }
        pub fn value_of_ptr(&self) -> &str {
            if self.ptr.is_null() {
            // if self.ptr == std::ptr::null() {
                panic!("NPE!!! Please init firstly");
            }
            unsafe { &*(self.ptr) }
        }
    }
    #[test]
    pub fn test_self_reference_with_unsafe() {
        let mut usr = UnsafeSelfRef::new("hi");
        // usr.value_of_ptr();
        usr.init();
        println!("value:{:?}, value_of_ptr:{:?}", usr.value(), usr.value_of_ptr());
    }
    #[test]
    pub fn test_self_reference_with_pin() {}
}

/// No `main` function found in crate `smart_pointers` [EO601]
fn main() {}