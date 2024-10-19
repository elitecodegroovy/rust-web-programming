use actix_web::{get, post, error, web, App, middleware::Logger, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use serde::Deserialize;
use futures::{future::ok, stream::once};

use derive_more::derive::{Display, Error};
use log::info;

use std::convert::Into;

#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

use std::fmt;

#[derive(Debug)]
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "圆的半径 {}", self.radius)
    }
}

use std::num::ParseIntError;
use std::str::FromStr;

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle{ radius: num }),
            Err(e) => Err(e),
        }
    }
}

async fn manual_hello() -> impl Responder {
    // 1 do something you like
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);
    // Ok
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // 2 literal
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // 3 type inference
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line
    // println!("{:?}", vec);

    // 4 conversion from and to operation
    let my_str = "Rust Web Programming";
    let my_string = String::from(my_str);

    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("我的数值是 {:?}", num);

    // 5 try_from
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    let circle = Circle { radius: 142857 };

    // 6 Parsing a String
    let parsed: i32 = "142857".parse().unwrap();
    let turbo_parsed = "999999".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;

    let radius2 = "    3 ";
    let circle2: Circle = radius2.parse().unwrap();
    println!("{:?}", circle2);

    let mut magic_number = 142857u32;
    let mut loop_count = 0;
    // Infinite loop
    loop {
        loop_count += 1;
        magic_number *= loop_count;
        if magic_number == 999999 {
            println!(" 142857u32 循环 {} 等于 999999 值", loop_count);
            // Exit this loop
            break;
        } else {
            magic_number = 142857;
        }
    }

    'tree1: loop {
        println!("进入第二个循环inner");
        'tree2: loop {
            println!("业务处理逻辑循环 inner");
            break 'tree2;
        }
        println!("outer 循环逻辑");
        break 'tree1;
        // 下面代码不会被执行
    }

    let mut c_number = 142857u32;
    let loop_number = loop {
        c_number += 142857;

        if c_number == 999999 {
            break 999999 / 142857;
        }
    };
    assert_eq!(loop_number, 7);

    let mut w_n = 142857u32;
    let mut while_count = 1u32;
    // Loop while `n` is less than 101
    while w_n != 999999 {
        while_count += 1;
        // 计数器加一
        w_n += 142857;
    }
    assert_eq!(while_count, 7);

    for _n in 1..=5 {
        // 循环 5 次
        println!("for循环n值{}", _n);
    }

    HttpResponse::Ok().body(format!("{}, {:?}, {}， radius： {}, parse string {}",
                                    mutable_binding, vec, my_string, circle.to_string(), sum))
}

#[allow(dead_code)]
#[derive(Debug)]
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
    let body = once(ok::<_, actix_web::Error>(web::Bytes::from_static(b"test")));

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