// structs3.rs
// Structs contain more than simply some data, they can also have logic, in this
// exercise we have defined the Package struct and we want to test some logic attached to it,
// make the code compile and the tests pass! If you have issues execute `rustlings hint structs3`

// this derive debug macro is a quick way to implement std::fmt:debug  which is like .toString().
// Basically, it means it prints all the attr of a struct. But you have to use {:?} to print a struct
// or {:#?} to pretty print. e.g. -> println!("rect1 is {:?}", rect1);
#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

// this impl is how you attach functions to a struct.
impl Package {
    // This is an ASSOCIATED FUNCTION, because it doesn't need self.
    // called via Package::new(fkjh). I guess these are static functions?
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            // use the panic macro to blow it up
            panic!("negative weight");
        } else {
            return Package {sender_country, recipient_country, weight_in_grams};
        }
    }

    // This is a METHOD, because it uses self.
    // called via package.is_international()
    fn is_international(&self) -> bool {
        return self.recipient_country != self.sender_country;
    }

    fn get_fees(&self, cents_per_kg: i32) -> i32 {
        return self.weight_in_grams * cents_per_kg;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_kg = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_kg), 4500);
    }
}
