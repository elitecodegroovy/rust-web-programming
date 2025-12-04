use actix_web::{web, App, HttpResponse, HttpServer, Responder};



async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Rust Web Programming")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on port 8080");
    HttpServer::new(|| {
        App::new()
            .route("/hi", web::get().to(hello))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}