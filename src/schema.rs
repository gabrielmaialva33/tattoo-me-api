// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 80]
        first_name -> Varchar,
        #[max_length = 80]
        last_name -> Varchar,
        #[max_length = 160]
        full_name -> Bpchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 40]
        username -> Varchar,
        #[max_length = 118]
        password -> Varchar,
        first_login_at -> Nullable<Timestamp>,
        last_login_at -> Nullable<Timestamp>,
        is_online -> Bool,
        is_email_verified -> Bool,
        is_active -> Bool,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}
