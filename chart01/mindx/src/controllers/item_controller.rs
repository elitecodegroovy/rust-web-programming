use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::item;
use crate::services::item_service;
use sqlx::PgPool;

pub async fn create_item_handler(
    pool: web::Data<PgPool>,
    item: web::Json<item::NewItem>
) -> impl Responder {
    match item_service::create_item(&pool, item.into_inner()).await {
        Ok(item) => HttpResponse::Created().json(item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_item_handler(
    pool: web::Data<PgPool>,
    item_id: web::Path<Uuid>
) -> impl Responder {
    match item_service::get_item(&pool, item_id.into_inner()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_all_items_handler(
    pool: web::Data<PgPool>
) -> impl Responder {
    match item_service::get_all_items(&pool).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_item_handler(
    pool: web::Data<PgPool>,
    item_id: web::Path<Uuid>,
    item: web::Json<item::NewItem>
) -> impl Responder {
    match item_service::update_item(&pool, item_id.into_inner(), item.into_inner()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_item_handler(
    pool: web::Data<PgPool>,
    item_id: web::Path<Uuid>
) -> impl Responder {
    match item_service::delete_item(&pool, item_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
