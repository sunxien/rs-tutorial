/// Rust Doc: https://course.rs/advance/smart-pointer/box.html
/// small data: allocate on stack
/// medium data: allocate on stack is better, but read is same
/// huge data: allocate on heap
#[cfg(test)]
#[allow(dead_code, unused)]
pub mod smart_pointers_test_cases {
    use std::fmt::{Display, Formatter};

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

    /// Rc: Reference count
    #[test]
    pub fn test_rc() {}

    /// Arc: Atomic reference count
    #[test]
    pub fn test_arc() {}

    /// RcCell:
    #[test]
    pub fn test_rc_cell() {
        println!()
    }

    /// ArcCell
    #[test]
    pub fn test_arc_cell() {
        println!()
    }
}

/// No `main` function found in crate `smart_pointers` [EO601]
fn main() {}