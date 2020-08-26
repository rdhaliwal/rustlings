// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)

// You define a modules for testing, and add this macro
#[cfg(test)]
mod tests {
    // You prefix each test with this macro to indicate it's a unit test
    #[test]
    fn you_can_assert() {
        // Assert a boolean condition
        assert!(10 > 1);

        // Assert two values are equal
        let thing = 10;
        assert_eq!(thing, 10);
    }
}
