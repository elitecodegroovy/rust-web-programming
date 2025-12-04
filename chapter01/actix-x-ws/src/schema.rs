// @generated automatically by Diesel CLI.

diesel::table! {
    r_posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 128]
        created_by -> Nullable<Varchar>,
    }
}
