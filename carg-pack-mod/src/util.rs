use crate::balle_balle_auth_ki_utilities::models::Cred;

pub fn op() {
    println!("Hello, world!");

    let user = Cred {
        username: String::from("admin"),
        password: String::from("admin"),
    };

    println!("User: {}", user.username);
}
