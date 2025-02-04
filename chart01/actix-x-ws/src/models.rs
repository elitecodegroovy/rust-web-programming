use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use diesel::prelude::*;
use serde::{Serialize};
use chrono;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Serialize, Default, Debug)]
#[diesel(table_name = crate::schema::r_posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RPosts {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_by: String,
}

impl Responder for RPosts {
    type Body = BoxBody;

    //noinspection DuplicatedCode
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::r_posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_by: &'a str,
}