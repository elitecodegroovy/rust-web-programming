diesel::table! {
    test_users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}