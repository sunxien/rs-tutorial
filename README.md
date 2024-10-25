#### 1. Difference between `String`, `&str`, `&String`?
1. `String` take ownership and malloc on Heap
2. `&str` is a slice reference of String
3. `&String` -> rust auto [deref] -> `&str`

Examples:
```
let x = String::from("abc"); // new heap => String
let y = &x; // Reference to String => &String
let z = &x[0, 5]; // String slice => &str 
```

#### 2. Difference between `self`,`&self`,`Self`, `&Self`?
1. `self` always used in method signature, that indicates the method belong to current object. ownership will move into the function.
2. `&self` same as `self`, but `&self` will pass a reference, so ownership won't move out.
3. `Self` a special type indicates this object, it always used in generic type.
4. `&Self` same as `Self`, but return a reference of this object.

Examples:
```
/// self VS &self
struct Hello(u32)
impl Hello {
    /// ownership will move into this method.
    pub fn move(self) {
        println!("....")
    }
    
    /// ownership won't move into this method.
    pub fn not_move(&self) {
       // TODO 
    }
}

/// Self VS &Self
struct Bye<T> {
    value: T,
}
impl<T> Bye<T> {
    pub fn say(value: T) -> Self {
        Bye { value }
    }
    pub fn say<'a>(value: T) -> &'a Self {
        &Bye { value }
    }
}
```
> Notice: If you want to modify object, please use `&mut self`.


#### 3. Difference between `pub()`, `pub(super)`, `pub(crate)`?
1. `pub` snippets are visible in root module.
2. `pub(crate)` snippets are visible in current crate.
3. `pub(self)` snippets are visible in current module. Same as `pub(in self)` or never use `pub`.
4. `pub(super)` snippets are visible in parent module. Same as `pub(in super)`.
5. `pub(in path)` ... e.g: `pub(in crate::outer_mod)`

Examples:
```
pub mod outer_mod {
    pub mod inner_mod {
        
        // this function is visible in `outer_mod`
        pub(in crate::outer_mod) fn outer_mod_visible_fn() {}
        
        // this function is visible in `outer_mod`. (Only used in Rust 2015)
        pub(in outer_mod) fn outer_mod_visible_fn_2015() {}

        // this function is visible in current crate.
        pub(crate) fn crate_visible_fn() {}

        // this function is visible in `outer_mod`
        pub(super) fn super_mod_visible_fn() {
            // `inner_mod_visible_fn` is visible as they are in the same mod: `inner_mod`。
            inner_mod_visible_fn();
        }

        // this function is visible in `inner_mod`. same as `private`
        pub(self) fn inner_mod_visible_fn() {}
    }
    
    pub fn foo() {
        inner_mod::outer_mod_visible_fn();
        inner_mod::crate_visible_fn();
        inner_mod::super_mod_visible_fn();

        // Compile error: Error! `inner_mod_visible_fn` is private
        //inner_mod::inner_mod_visible_fn();
    }
}
```