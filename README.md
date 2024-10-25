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

#### 2. Difference between `Self`, `&self`, `&Self`?
1. `Self`
2. `&self`
3. `&Self`

Examples:
```
code here
```


#### 3. Difference between `pub()`, `pub(super)`, `pub(crate)`?
1. `pub`
2. `pub(super)`
3. `pub(crate)`

Examples:
```