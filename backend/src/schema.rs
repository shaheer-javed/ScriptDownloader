// @generated automatically by Diesel CLI.

diesel::table! {
    installation (id) {
        id -> Int4,
        order_number -> Varchar,
        setup_name -> Varchar,
        order_status -> Varchar,
        username -> Varchar,
        domain -> Varchar,
    }
}
