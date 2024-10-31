/// Rust Doc: https://course.rs/advance/smart-pointer/box.html
/// small data: allocate on stack
/// medium data: allocate on stack is better, but read is same
/// huge data: allocate on heap
#[cfg(test)]
#[allow(dead_code, unused)]
pub mod smart_pointers_test_cases {
    use std::fmt::{Display, Formatter, Pointer};
    use std::ops::Deref;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::thread;
    use std::thread::{Builder, Thread};
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
        // let mut i = 1; // borrowed outlive
        // let mut i = Rc::new(1); // compile error: `Rc<i32>` cannot be sent between threads safely
        // let mut i = Arc::new(1);
        // let mut threads = Vec::new();
        // for k in '1'..='8' {
        //     let mut prefix: String = "thread-".into();
        //     prefix.push(k);
        //     let t = thread::Builder::new().name(prefix).spawn(|| {
        //         i = (i.deref() + 1).into();
        //         println!("Current thread: {}", thread::current().name().unwrap());
        //     }).unwrap();
        //     threads.push(t);
        // }
        // for t in threads {
        //     t.join().unwrap();
        // }
    }

    /// RcCell:
    #[test]
    pub fn test_rc_cell() {}

    /// ArcCell
    #[test]
    pub fn test_arc_cell() {}
}

/// No `main` function found in crate `smart_pointers` [EO601]
fn main() {}