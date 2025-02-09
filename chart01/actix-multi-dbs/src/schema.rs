// @generated automatically by Diesel CLI.

diesel::table! {
    t_infra_user (id) {
        id -> Int8,
        real_name -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    a_general_order(id) {
        id -> Int8,
        work_order_type -> Varchar,
        work_order_title -> Varchar,
        work_order_text -> Varchar,
    }
}