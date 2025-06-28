mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("added to wait list!");
        }
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // private field
    }

    impl Breakfast {
        // Public constructor for Breakfast
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// pub fn eat_at_restaurant() {
//     // Using the front_of_house module
//     // absolute path
//     crate::front_of_house::hosting::add_to_wait_list();
//     // relative path
//     front_of_house::hosting::add_to_wait_list();
// }

use crate::front_of_house::hosting;

// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_wait_list();
//     }
// }

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    // let mut meal = back_of_house::Breakfast::summer("Rye");
    // meal.toast = String::from("Wheat");
    // println!("I'd like {} toast please", meal.toast);

    // Trying to access the private field (this will fail)
    // println!("I'd like {} please", meal.seasonal_fruit);

    // meal.seasonal_fruit = String::from("blueberries");
    // println!(
    //     "I'd like {} toast with {} fruit",
    //     meal.toast, meal.seasonal_fruit
    // );
}
