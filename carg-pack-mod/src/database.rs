pub enum Status {
    CONNECTED,
    AUTHENTICATED,
    DISCONNECTED,
}
pub fn connect_to_db() -> Status {
    println!("Connecting to the database...");
    Status::CONNECTED
}
pub fn get_user() {
    println!("Getting user from the database...");
}
