use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    username: String,
    password: String,
    first_login_at: String,
    created_at: String,
    updated_at: String,
}