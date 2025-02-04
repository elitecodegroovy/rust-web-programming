use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;

use chrono::prelude::{Utc};

use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<models::NewUser>,
) -> HttpResponse {
    use schema::test_users::dsl::*;

    let mut conn = pool.try_get().expect("Failed to get DB connection");

    let now = Utc::now().naive_utc();
    let new_example = models::NewExample {
        created_at: now,
        updated_at: Some(now),
    };

    let database_record = diesel::insert_into(schema::r_examples::table)
        .values(&new_example)
        .get_result::<models::Example>(&mut conn);
    println!("Inserted example: {:?}", database_record);

    let result = diesel::insert_into(test_users)
        .values(&*new_user)
        .get_result::<models::User>(&mut conn);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
    use schema::test_users::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");

    match test_users.load::<models::User>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> HttpResponse {
    use schema::test_users::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");

    match test_users.find(user_id.into_inner()).first::<models::User>(&mut conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    updated_user: web::Json<models::NewUser>,
) -> HttpResponse {
    use schema::test_users::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");

    let result = diesel::update(test_users.find(user_id.into_inner()))
        .set((
            name.eq(&updated_user.name),
            email.eq(&updated_user.email),
        ))
        .get_result::<models::User>(& mut conn);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> HttpResponse {
    use schema::test_users::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");

    match diesel::delete(test_users.find(user_id.into_inner())).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/test_users", web::post().to(create_user))
            .route("/test_users", web::get().to(get_users))
            .route("/test_users/{id}", web::get().to(get_user))
            .route("/test_users/{id}", web::put().to(update_user))
            .route("/test_users/{id}", web::delete().to(delete_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}