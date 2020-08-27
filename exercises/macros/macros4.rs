// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

#[macro_use]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // Separate each macro arm with a semi-color
    // Have to use tt, not expr if i want to forward values on to another macro
    // tt is token tree. More details here: https://doc.rust-lang.org/reference/macros-by-example.html
    ($val:tt) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
