#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Using a semicolon after `mod front_of_house` rather than using 
// a block tells Rust to load the contents of the module from 
// another file with the same name as the module. 
mod front_of_house;

// Demonstrating putting `hosting` into scope with `use` so that
// methods can be called more succinctly (as though they were local)
//
// Interesting that you can use `use` to bring into scope even within the
// same file.
use crate::front_of_house::hosting;


fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // We can also construct relative paths that begin in the parent 
        // module by using super at the start of the path. This is analogous 
        // to starting a filesystem path with the "../" syntax. 
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        // Even if you make the struct public, the inner fields are
        // still private unless explicitly made public also
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Note that because back_of_house::Breakfast has a private field, 
        // the struct needs to provide a public associated function that 
        // constructs an instance of Breakfast (we’ve named it summer here). 
        // 
        // If Breakfast didn’t have such a function, we couldn’t create an instance of 
        // Breakfast in eat_at_restaurant (further down) because we couldn’t set the 
        // value of the private seasonal_fruit field in eat_at_restaurant.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In contrast to structs, if we make an enum public, all of its 
    // variants are then public. We only need the pub before the enum keyword.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();


    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    //
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}