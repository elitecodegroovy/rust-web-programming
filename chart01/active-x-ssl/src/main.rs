use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hi there!")
}

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("nopass.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new().app_data(web::Data::new(AppState {
            app_name: String::from("FastlyActix"),
        })).service(index)
    })
    .bind_openssl("0.0.0.0:8080", builder)?
    .run()
    .await
}