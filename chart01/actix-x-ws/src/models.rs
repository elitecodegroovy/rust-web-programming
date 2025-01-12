use diesel::prelude::*;
use serde::{Serialize};

#[allow(dead_code)]
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::r_posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RPosts {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::r_posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}