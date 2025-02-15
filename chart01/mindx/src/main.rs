mod bootstrap;
mod db;
mod controllers;
mod models;
mod services;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    bootstrap::bootstrap().await.expect("bootstrap: panic message");

    let pool = db::establish_connection().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::post().to(controllers::item_controller::create_item_handler))
            .route("/items/{id}", web::get().to(controllers::item_controller::get_item_handler))
            .route("/items", web::get().to(controllers::item_controller::get_all_items_handler))
            .route("/items/{id}", web::put().to(controllers::item_controller::update_item_handler))
            .route("/items/{id}", web::delete().to(controllers::item_controller::delete_item_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}