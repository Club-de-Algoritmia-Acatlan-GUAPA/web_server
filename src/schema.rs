// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        is_validated -> Bool,
        creation_date -> Int4,
        username -> Varchar,
        user_role -> Int4,
    }
}
