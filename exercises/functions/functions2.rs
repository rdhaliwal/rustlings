// functions2.rs

// variable name: type.
// let thing: u8
// fn thing(some: u8) {}

fn main() {
    call_me(3);
}

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
