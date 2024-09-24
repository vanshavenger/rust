// use carg_pack_mod::{authenticate, balle_balle_auth_ki_utilities::models::Cred, util};
use crate::garden::vegetables::Asparagus;

use std::io::Result;
use std::fmt::Result as FmtResult;

pub mod garden;


fn main() {
    // println!("Hello, world!");

    // let user = Cred {
    //     username: String::from("admin"),
    //     password: String::from("admin"),
    // };

    // authenticate(&user);

    // util::op();

    let plant = Asparagus::TYPEA;
    println!("{:?}", plant);

    

    
}
