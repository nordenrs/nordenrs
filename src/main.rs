use sea_orm::Database;
//use actix_files::Files as Fs;
use actix_web::{
    middleware, web, App, HttpServer,
};
use listenfd::ListenFd;
//use migration::{Migrator, MigratorTrait};
use std::env;
//use tera::Tera;
use nordenrs::routes::api::route;
use nordenrs::configures::options::AppState;

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    //let server_url = format!("{host}:{port}");
    
    let server_url = format!("0.0.0.0:{port}");
println!("Database URL: {}", db_url);
println!("Server URL: {}", server_url);
    // establish connection to database and apply migrations
    // -> create post table if not exists
    let conn = Database::connect(&db_url).await.unwrap();

    //Migrator::up(&conn, None).await.unwrap();

    // load tera templates and build app state
    let state = AppState { conn };

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            //.service(Fs::new("/static", "./api/static"))
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(route)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}


pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

