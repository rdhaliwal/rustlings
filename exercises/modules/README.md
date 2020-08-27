### Modules

In this section we'll give you an introduction to Rust's module system.

#### Book Sections

- [The Module System](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)


- Packages is one or more crates.
    - Packages have a cargo.toml
    - Crates are binaries or libraries.
        - Modules are groups of code within a crate
            - Modules are defined by the `mod` keyword
            - You can have modules within modules

- Everything is private by default
    - Crates, modules, functions, enums, etc.
    - you have to add the `pub` prefix to make things public.
- If you want to import another module, instead of writing the absolute path, you can import them with `use`
```rust
// Aliases hosting to local scope, so you can freely call hosting and it's children
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
- It's convention to import one above what you actually call.
- We _could_ import `crate::front_of_house::hosting::add_to_waitlist`, but it's better to just import `crate::front_of_house::hosting`
so that when we call `hosting::add_to_waitlist` it's a bit clearer.
- We can rename imports with the `as` keyword:
```rs
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result { }
fn function2() -> IoResult<()> { }
```


- We can import multiple things at once:
```rs
use std::cmp::Ordering;
use std::io;

// Equivalent
use std::{cmp::Ordering, io};

// This makes Ordering and all other things imported.
use std::cmp::*;
```

- Module names are related to file names. So whatever goes in the filename and matches the module within it, is importable (assuming it's public)
