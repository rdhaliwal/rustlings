Ownership:
    - variables for non primitive types have pointers to the actual data

```rust
let s1 = String::from("hello");
let s2 = s1;
```

s1 and s2 are pointing to the same block of memory storing 'hello'

Because of this, when a variable goes out of scope, or has something else point to it
Rust tracks this and invalidates the original assigner, and only lets the second thing use it.

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```
Would fail, because s1 has been invalidated.

We can do a clone to deep clone the value:
```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);
```
And now s1 and s2 can be freely read and manipulated independetly.

For scope, once the variable is out of scope it can't be accessed.
Or if it's passed as a variable, then it's borrowed and the original is invalidated.
```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
// moved, so nothing happens. s1 goes out of scope and is dropped.
```

It'd be pretty annoying having to constantly redeclare variables when using functions.
So, we can use references:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.
```
`s` is of type 'string reference'
`&s1` refers to the value of s1, but does not own it.
    - Because it does not own it, it will not be dropped when out of scope.

Just like variables are immutable by default, we must specify if a reference is mutable
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
But you can only have 1 mutable refence per scope. this fails:
```rust
let mut s = String::from("hello");

// This fails.
let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}, and {}", r1, r2);
```

This also fails. even if it's only one mutable reference but multiple
```rust
let mut s = String::from("hello");

// this also fails. even if it's only one mutable reference but multiple
// references, no go.
let r1 = &s;
let r2 = &s;
let r3 = &mut s;

println!("{}, {}, and {}", r1, r2, r3);
```

HOWEVER! If the immutable references are dropped out of scope before the mutable reference, then it's okay. So this can work, because r1 and r2 are dropped before r3:
```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{}, {}, and {}", r1, r2);

let r3 = &mut s;

println!("{}", r3);
```

This is part of Rusts whole memory safety thing, and helps prevent data races.

## Slices:

We can create slices of strings (or arrays) with the following syntax
```rust
#![allow(unused_variables)]
    fn main() {
    let s = String::from("hello");

    let slice = &s[0..2]; // 0 to 2
    let slice = &s[..2]; // 0 to 2 equivalent

    let slice = &s[3..len]; // 3 to end
    let slice = &s[3..]; // 3 to end, equivalent

    let slice = &s[0..len]; // start to end
    let slice = &s[..]; // start to end, equivalent

}
```
