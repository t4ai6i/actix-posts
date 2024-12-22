use crate::handler::data;
use actix_web::{get, HttpResponse, Responder};

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
