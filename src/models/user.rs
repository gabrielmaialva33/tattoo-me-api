use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    full_name: String,
    email: String,
    username: String,
    password: String,
    first_login_at: Option<NaiveDateTime>,
    last_login_at: Option<NaiveDateTime>,
    is_online: bool,
    is_email_verified: bool,
    is_active: bool,
    is_deleted: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}
