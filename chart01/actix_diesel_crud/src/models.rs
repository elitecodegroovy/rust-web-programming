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