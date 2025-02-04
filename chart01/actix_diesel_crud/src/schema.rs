diesel::table! {
    test_users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}
diesel::table! {
    r_examples (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}