// variables3.rs

// let is a constant. All variables are constant by default.
// add a mut will make it a variable.
// However, there is a const that is always a constant, and won't accept const

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
