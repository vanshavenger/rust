use carg_pack_mod::{authenticate, balle_balle_auth_ki_utilities::models::Cred, util};

fn main() {
    println!("Hello, world!");

    let user = Cred {
        username: String::from("admin"),
        password: String::from("admin"),
    };

    authenticate(&user);

    util::op();
}
