use crate::database;

pub fn login(cred: &models::Cred) {
    database::get_user()
}

pub mod models;
