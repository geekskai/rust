mod front_of_house {
    mod hosting {
        fn add_to_wait_list() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Using the front_of_house module
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list();
    // relative path
    front_of_house::hosting::add_to_wait_list();
}
