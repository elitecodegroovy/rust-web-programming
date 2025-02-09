mod schema;

use diesel::pg::PgConnection;
use diesel::mysql::MysqlConnection;
#[allow(unused_imports)]
use r2d2::{Pool, PooledConnection};
use dotenv::dotenv;
use std::env;

type PgPool = Pool<ConnectionManager<PgConnection>>;
type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub struct DbState {
    pub pg_pool: PgPool,
    pub mysql_pool: MysqlPool,
}

pub fn establish_connection() -> DbState {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secondary_database_url = env::var("SECONDARY_DATABASE_URL").expect("SECONDARY_DATABASE_URL must be set");

    // let max_conn = env::var("MAX_CONNECTION_SIZE").expect("MAX_CONNECTION_SIZE must be set");
    // let db_conn_size = max_conn.parse::<u32>().unwrap();

    let pg_manager = ConnectionManager::<PgConnection>::new(database_url);
    let mysql_manager = ConnectionManager::<MysqlConnection>::new(secondary_database_url);

    let pg_pool = Pool::builder().max_size(std::env::var("POOL_MAX_SIZE")
                                                                                .ok()
                                                                                .and_then(|s| s.parse::<u32>().ok())
                                                                                .unwrap_or(20)) // Default to 10 if not set
                                                    .build(pg_manager).expect("Failed to create PG pool");
    println!("init pg DB");
    let mysql_pool = Pool::builder().build(mysql_manager).expect("Failed to create MySQL pool");
    println!("init mysql DB");
    DbState { pg_pool, mysql_pool }

}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, QueryableByName, PartialEq, Debug)]
#[diesel(table_name = crate::schema::t_infra_user)]
struct User {
    id: i64,
    real_name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, PartialEq, Debug)]
#[diesel(table_name = crate::schema::a_general_order)]
struct WorkerOrder {
    pub id: i64,
    pub work_order_type: String,
    pub work_order_title: String,
    pub work_order_text: String,
}

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;

#[derive(Deserialize)]
struct PaginationParams {
    page: Option<i64>,
    page_size: Option<i64>,
}

async fn get_users(db: web::Data<DbState>, query_params: web::Query<PaginationParams>) -> impl Responder {

    // Set default page number and page size if not provided
    let page = query_params.page.unwrap_or(1);
    let page_size = query_params.page_size.unwrap_or(10);

    // Calculate OFFSET based on page number and page size
    let offset = (page - 1) * page_size;

    let mut conn = db.pg_pool.get().unwrap();

    // Write your raw SQL query with pagination (LIMIT and OFFSET)
    let query = format!(
        "SELECT id, real_name, email FROM t_infra_user ORDER BY id LIMIT {} OFFSET {}",
        page_size, offset
    );

    // Execute the query and map it to Post struct
    let results = sql_query(query)
        .load::<User>(&mut conn)
        .expect("Error loading users");

    HttpResponse::Ok().json(results)
}


async fn get_orders(db: web::Data<DbState>) -> impl Responder {
    // use crate::schema::a_general_order::dsl::*;

    let mut conn = db.mysql_pool.get().unwrap();
    let pure_sql = "SELECT id, work_order_type, work_order_title, work_order_text FROM a_general_order ORDER BY id".to_string();
    let results = sql_query(pure_sql).load::<WorkerOrder>(&mut conn).unwrap();

    HttpResponse::Ok().json(results)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_state = web::Data::new(establish_connection());

    HttpServer::new(move || {
        App::new()
            .app_data(db_state.clone())
            .route("/users", web::get().to(get_users))
            .route("/orders", web::get().to(get_orders))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
