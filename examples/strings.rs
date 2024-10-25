fn main() {
    let s1 = String::from("hello"); // malloc on Heap
    println!("Value(String): {:?}", s1);

    let s2 = &s1; // &String means a reference to s1(String)
    println!("Value(&String): {:?}", s2);

    let s3 = &s1[0..s1.len()];
    println!("Value(&str): {:?}", s3); // &str means a slice reference

    // Rust will auto deref &String to &str
}