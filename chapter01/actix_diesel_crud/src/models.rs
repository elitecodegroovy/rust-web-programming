use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::test_users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::r_examples)]
pub struct NewExample {
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[allow(dead_code)]
#[derive(Queryable, Debug)]
pub struct Example {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}