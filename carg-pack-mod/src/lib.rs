// Root library file for the crate

#![allow(dead_code, unused_variables)]

mod database;

pub mod balle_balle_auth_ki_utilities;

use balle_balle_auth_ki_utilities::login;
use balle_balle_auth_ki_utilities::models::Cred;
use database::{connect_to_db, Status};

pub fn authenticate(cred: &Cred) {
    if let Status::CONNECTED = connect_to_db() {
        login(cred);

        println!("User connected");
    } else {
        println!("User not connected");
    }
}

pub mod util;
