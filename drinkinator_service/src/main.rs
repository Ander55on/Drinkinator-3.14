use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use responses::responses::response_error_server_error;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::convert::Infallible;
use std::net::SocketAddr;

pub mod database;
mod models;
mod responses;
mod routes;
use routes::route::route;

async fn handle_request(
    req: Request<Body>,
    pool: Pool<Sqlite>,
) -> Result<Response<Body>, Infallible> {
    let path = String::from(req.uri().path());
    let method = req.method().clone();
    let body = req.into_body();

    let body_bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error: {e}");
            return Ok(response_error_server_error());
        }
    };

    let response = route(path, method, body_bytes).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        response_error_server_error()
    });

    Ok(response)
}

async fn create_database_schema(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS drinks (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ingredients (
        name TEXT PRIMARY KEY
    )"
    )
    .execute(pool)
    .await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS ingredient_instruction (
        ingredient_name TEXT NOT NULL,
        drink_id INTEGER NOT NULL,
        unit TEXT NOT NULL,
        quantity INTEGER NOT NULL,
        FOREIGN KEY(ingredient_name) REFERENCES ingredients(name),
        FOREIGN KEY(drink_id) REFERENCES drinks(id),
        PRIMARY KEY(ingredient_name, drink_id)
    )").execute(pool).await?;

    Ok(())
    
}

#[tokio::main]
async fn main() {
    let db_url = String::from("data/drinkinator.db");

    if !Sqlite::database_exists(&db_url).await.unwrap() {
        println!("Creating database");
        Sqlite::create_database(&db_url)
            .await
            .expect("Failed to create db");
    } else {
        println!("Database exists");
    }

    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to establish db connection");

    println!("Database connection established!");

    create_database_schema(&pool)
        .await
        .expect("Failed to create tables");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(move |_conn: &hyper::server::conn::AddrStream| {
        let pool = pool.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| handle_request(req, pool.clone()))) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
