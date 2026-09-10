mod front_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {}

#[allow(dead_code)]
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        // super is used to derive a fn/code from the outside scope of a module in the same package file
        super::deliver_order();
    }

    fn cook_order() {}
}

#[allow(dead_code)]
mod customer {

    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    // it better to use the absolute path in code chunks
    // crate::front_of_the_house::hosting::add_to_waitlist();

    // relative-path: this is used for dynamic typing but absolute is preferred
    // front_of_the_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
