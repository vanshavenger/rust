// Root library file for the crate

#![allow(dead_code, unused_variables)]

// mod database;

// pub mod balle_balle_auth_ki_utilities;

// use balle_balle_auth_ki_utilities::login;
// use balle_balle_auth_ki_utilities::models::Cred;
// use database::{connect_to_db, Status};

// pub fn authenticate(cred: &Cred) {
//     if let Status::CONNECTED = connect_to_db() {
//         login(cred);

//         println!("User connected");
//     } else {
//         println!("User not connected");
//     }
// }

// pub mod util;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

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
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

}

fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);
}