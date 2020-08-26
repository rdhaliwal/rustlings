// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` for hints :)

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn divide_by_larger_number(a: u8, b: u8) -> u8 {
    if b == 0 {
        panic!("Divide by zero")
    } else if a > b {
        return b / a;
    } else {
        return a / b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));
    }

    #[test]
    #[should_panic]
    fn this_test_panics() {
        assert_eq!(divide_by_larger_number(10, 0), 1);
    }
}
