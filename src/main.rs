use actix_posts::handler::api::{
    api_create, api_delete, api_index, api_not_found, api_show, api_update,
};
use actix_posts::handler::routes::{create, destroy, edit, index, new, not_found, show, update};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::SessionMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use env_logger::Env;
use std::io::Result;

fn build_cookie_session_middleware(key: Key) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), key).build()
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let key = Key::generate();
    let message_store = SessionMessageStore::default();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    HttpServer::new(move || {
        let tera = tera::Tera::new("templates/**/*.html").unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .service(index)
            .service(new)
            .service(create)
            .service(edit)
            .service(update)
            .service(destroy)
            .service(show)
            .service(
                web::scope("/api")
                    .service(api_index)
                    .service(api_show)
                    .service(api_create)
                    .service(api_update)
                    .service(api_delete)
                    .default_service(web::to(api_not_found)),
            )
            .default_service(web::to(not_found))
            .wrap(Logger::default())
            .wrap(message_framework.clone())
            .wrap(build_cookie_session_middleware(key.clone()))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
