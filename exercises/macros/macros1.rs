// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// Have to define macro before being used.
#[macro_use]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
