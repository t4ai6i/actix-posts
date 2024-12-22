use actix_posts::handler::routes::index;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use std::io::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| App::new().service(index).wrap(Logger::default()))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
