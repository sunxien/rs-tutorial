/// Before Rust 1.30 use mod.rs define multi-modules.
/// And the `mod.rs` will contain all submodules.
/// - src/lib.rs (or main.rs)
/// - src/module2/mod.rs   (use, or invoke submodules)
/// - src/module2/submodule1.rs
/// - src/module2/submodule2.rs

mod module2_submodule1;
mod module2_submodule2;

pub fn print(){
    println!("this is module2");
    module2_submodule1::m2s1::print_m2s1();
    module2_submodule2::m2s2::print_m2s2();
}