use crate::handler::data;
use crate::handler::data::Message;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{DateTime, Local};
use serde::Deserialize;
use tera::Context;

#[get("/posts")]
pub async fn index(tmpl: web::Data<tera::Tera>) -> impl Responder {
    let posts = data::get_all();
    let mut context = Context::new();
    context.insert("posts", &posts);
    let body_str = tmpl.render("index.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/{id}")]
pub async fn show(tmpl: web::Data<tera::Tera>, info: web::Path<i32>) -> impl Responder {
    let info = info.into_inner();
    let post = data::get(info);
    let mut context = Context::new();
    context.insert("post", &post);
    let body_str = tmpl.render("show.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/new")]
pub async fn new(tmpl: web::Data<tera::Tera>) -> impl Responder {
    let mut context = Context::new();
    let post = Message {
        ..Default::default()
    };
    context.insert("action", "create");
    context.insert("post", &post);
    context.insert("button", "投稿");
    let body_str = tmpl.render("form.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/{id}/edit")]
pub async fn edit(tmpl: web::Data<tera::Tera>, info: web::Path<i32>) -> impl Responder {
    let info = info.into_inner();
    let post = data::get(info);
    let mut context = Context::new();
    context.insert("action", "update");
    context.insert("post", &post);
    context.insert("button", "更新");
    let body_str = tmpl.render("form.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[derive(Deserialize, Debug)]
pub struct CreateForm {
    id: i32,
    posted: String,
    sender: String,
    content: String,
}

#[post("/posts/create")]
pub async fn create(params: web::Form<CreateForm>) -> impl Responder {
    let now: DateTime<Local> = Local::now();
    let mut message = Message {
        id: 0,
        posted: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };
    message = data::create(message);
    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
}

#[post("/posts/update")]
pub async fn update(params: web::Form<CreateForm>) -> impl Responder {
    let message = Message {
        id: params.id,
        posted: params.posted.clone(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };
    data::update(&message);
    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
}

#[get("/posts/{id}/delete")]
pub async fn destroy(info: web::Path<i32>) -> impl Responder {
    let info = info.into_inner();
    data::remove(info);
    web::Redirect::to("/posts").see_other()
}

/// Handles requests to non-existent routes by returning a 404 Not Found response.
///
/// This asynchronous handler responds with an HTTP 404 status code and a simple
/// message indicating that the requested page was not found.
///
/// # Returns
/// An implementation of [`Responder`] that contains an HTTP 404 response
/// with a body message "Page Not Found!".
pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Page Not Found!")
}
