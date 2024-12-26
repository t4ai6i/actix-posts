use actix_posts::handler::routes::{create, destroy, edit, index, new, not_found, show, update};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::io::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(new)
            .service(create)
            .service(edit)
            .service(update)
            .service(destroy)
            .service(show)
            .default_service(web::to(not_found))
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
