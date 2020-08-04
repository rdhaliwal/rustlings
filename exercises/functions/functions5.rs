// functions5.rs

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num

    // Either you explicitly return and use the semi-colon
    //   return thing;
    // or you just use the variable, without the semi-colon
    //   thing
}
