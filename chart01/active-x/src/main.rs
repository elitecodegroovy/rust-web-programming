use actix_web::{get, post, Error,error, web, App, middleware::Logger, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use serde::Deserialize;
use futures::{future::ok, stream::once};

use derive_more::derive::{Display, Error};
use log::info;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hi there!")
}

struct AppState {
    app_name: String,
}

//noinspection ALL
#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

#[get("/usersV2/{user_id}/{friend}")] // <- define path parameters
async fn index2(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}

#[derive(Deserialize)]
struct UserInfo {
    user_name: String,
    password: String,
}

#[get("/getQueryParams")]
async fn get_query_params(info: web::Query<UserInfo>) -> String {
    format!("Welcome {}! , password: {}!", info.user_name, info.password)
}

#[post("/submitForm")]
async fn submit_form(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}, {}", info.user_id, info.friend))
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}


#[derive(Debug, Display, Error)]
#[display("my error: {name}")]
pub struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

#[get("/index_for_err")]
async fn index_for_err() -> Result<&'static str, MyError> {
    let err = MyError { name: "test error" };
    info!("{}", err);
    Err(err)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe { std::env::set_var("RUST_LOG", "info"); }
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            // 4K * 225
            .limit(4096*225)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(AppState {
                    app_name: String::from("FastlyActix"),
                }))
            .service(
                web::scope("/gapi")
                    .app_data(json_config)
                    .route("/hi", web::get().to(manual_hello))
                    .route("/index2", web::get().to(manual_hello))
                    .service(submit_form)
                    .service(index)
                    .service(index2)
                    .service(get_query_params)
                    .service(stream)
                    .service(index_for_err)
            )

    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}