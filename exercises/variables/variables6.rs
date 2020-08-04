// variables6.rs
// consts must always have a type annotation
/*
Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
128-bit	i128	u128

so i8 = -127 > 128. u8 = 0 - 256
*/

const NUMBER: i8 = 30;
// const NUMBER: u8 = 256;

fn main() {
    println!("Number {}", NUMBER);
}
