// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

#[macro_use]
pub mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    // Not sure how, but somehow the scope is happy WITHOUT using `macros`?
    my_macro!();
}
