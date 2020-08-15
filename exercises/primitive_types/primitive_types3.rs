// primitive_types3.rs

// This declares an empty array of size 100, for i16
// let a: [i16; 100];
// This creates an array of size 100, with each value being 123
// let a =  [123; 100];

fn main() {
    // This declares an empty array of size 100, for i16
    // let a: [i16; 100];
    // This creates an array of size 100, with each value being 123
    let a =  [123; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
