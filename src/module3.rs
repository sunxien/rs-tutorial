/// Since Rust 1.31 use {same_name}.rs define multi-modules.
/// The `{same_name}.rs` is located at root crate, and it will contain all submodules.
/// - src/lib.rs (or main.rs)
/// - src/module.rs  (use, or invoke submodules)
/// - src/module/submodule1.rs
/// - src/module/submodule2.rs

pub mod module3_submodule1;

pub fn print(){
    println!("this is module1");
    module3_submodule1::m3s1::print_m3s1();
}