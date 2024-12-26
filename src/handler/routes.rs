use crate::handler::data;
use crate::handler::data::Message;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{DateTime, Local};
use serde::Deserialize;

#[get("/posts")]
pub async fn index() -> impl Responder {
    let posts = data::get_all();
    let mut body_str = String::new();
    body_str += include_str!("../../static/header.html");
    posts.iter().for_each(|post| {
        body_str += &format!("<div><a href=\"/posts/{}\">", post.id);
        body_str += &format!("<div>{} {}</div>", post.sender, post.posted);
        body_str += &format!("<div><p>{}</p></div>", post.content.replace("\n", "<br />"));
        body_str += "</a></div>";
    });
    body_str += "<div><a href=\"/posts/new\">作成</a></div>";
    body_str += include_str!("../../static/footer.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/{id}")]
pub async fn show(info: web::Path<i32>) -> impl Responder {
    let info = info.into_inner();
    let post = data::get(info);
    let mut body_str = String::new();
    body_str += include_str!("../../static/header.html");
    body_str += "<div>";
    if post != Message::default() {
        body_str += &format!("<div>投稿者：{}</div>", post.sender);
        body_str += &format!("<div>投稿日時：{}</div>", post.posted);
        body_str += &format!(
            "<div>投稿内容：<br />{}</div>",
            post.content.replace("\n", "<br />")
        );
        body_str += &format!("<div><a href=\"/posts/{}/edit\">編集</a> ", info);
        body_str += &format!("<a href=\"/posts/{}/delete\">削除</a></div>", info);
    } else {
        body_str += "見つかりません。";
    }
    body_str += "</div>";
    body_str += "<div><a href=\"/posts\">一覧へ</a></div>";
    body_str += include_str!("../../static/footer.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/new")]
pub async fn new() -> impl Responder {
    let mut body_str = String::new();
    body_str += include_str!("../../static/header.html");
    body_str += include_str!("../../static/form.html");
    body_str = body_str.replace("{{action}}", "create");
    body_str = body_str.replace("{{id}}", "0");
    body_str = body_str.replace("{{posted}}", "");
    body_str = body_str.replace("{{sender}}", "");
    body_str = body_str.replace("{{content}}", "");
    body_str = body_str.replace("{{button}}", "登録");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/{id}/edit")]
pub async fn edit(info: web::Path<i32>) -> impl Responder {
    let info = info.into_inner();
    let post = data::get(info);
    let mut body_str: String = "".to_string();
    body_str += include_str!("../../static/header.html");
    body_str += include_str!("../../static/form.html");
    body_str += include_str!("../../static/footer.html");
    body_str = body_str.replace("{{action}}", "update");
    body_str = body_str.replace("{{id}}", &post.id.to_string());
    body_str = body_str.replace("{{posted}}", &post.posted);
    body_str = body_str.replace("{{sender}}", &post.sender);
    body_str = body_str.replace("{{content}}", &post.content);
    body_str = body_str.replace("{{button}}", "更新");
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
