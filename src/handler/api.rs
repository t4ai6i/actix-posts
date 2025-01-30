//! This module contains definitions for API-related response structures.
//!
//! The primary purpose of the module is to define reusable data types for representing
//! structured responses from the application, such as API payloads or internal result processing.
//!
//! ## Main Components
//!
//! - **`ResponseContent`**
//!   - An enum representing the kind of response being returned. It supports different
//!     variants to handle various response scenarios such as collections, single items,
//!     error reasons, and empty results.
//!   - Derived with `Serialize` for seamless serialization (e.g., to JSON) and `Debug`
//!     for debugging purposes.
//!
//! - **`ApiResponse`**
//!   - A struct representing the overall structure of an API response. Contains a status field
//!     to indicate the response status (e.g., success or failure) alongside the `ResponseContent`.

use crate::handler::data;
use crate::handler::data::{get, get_all, Message};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::Local;
use serde::{Deserialize, Serialize};

/// Represents the content of an API response.
///
/// This enum provides a flexible structure for representing different data types in responses.
/// It includes multiple variants specifying whether the response contains a list of items,
/// a single item, an error reason, or no payload at all.
///
/// ### Variants
/// - `Items(Vec<Message>)`: Represents a collection of `Message` objects.
/// - `Item(Message)`: Represents a single `Message` object.
/// - `Reason(String)`: Represents a textual description of an error or explanation.
/// - `None`: Represents the absence of content or data.
///
/// ### Derived Traits
/// - `Serialize`: Allows the enum to be easily serialized (e.g., to JSON) via Serde.
/// - `Debug`: Enables debugging with the `{:?}` formatter.
#[derive(Serialize, Debug)]
enum ResponseContent {
    Items(Vec<Message>),
    Item(Message),
    Reason(String),
    None,
}

/// Represents the structure of an API response.
///
/// The `ApiResponse` is a wrapper to provide a consistent API response format,
/// containing two fields:
///
/// - `status`: A string to specify the response status (e.g., "success", "error").
/// - `result`: The data of the response, represented by [`ResponseContent`].
///
/// ### Derived Traits
/// - `Serialize`: Enables the struct to be serialized (e.g., to JSON).
/// - `Debug`: Allows for inspection using the `{:?}` formatter.
#[derive(Serialize, Debug)]
struct ApiResponse {
    status: String,
    result: ResponseContent,
}

#[derive(Deserialize)]
struct Queries {
    format: Option<String>,
}

/// Handles requests to undefined API routes.
///
/// This function returns an HTTP `404 Not Found` response with a JSON payload
/// indicating that the requested API endpoint does not exist.
///
/// ### Response Structure
/// - **Status**: `"Error"`
/// - **Reason**: `"API not found"`
///
/// ### Example Response Payload (JSON)
/// ```json
/// {
///     "status": "Error",
///     "result": {
///         "Reason": "API not found"
///     }
/// }
/// ```
///
/// ### Usage
/// Typically, this function is used as a handler in cases where the route
/// requested by the client is invalid or not defined.
///
/// ### Returns
/// - Responds with an object implementing the `Responder` trait, containing
///   the `ApiResponse` serialized as JSON.
///
/// ### Function Context
/// This function integrates with Actix Web's async framework and uses
/// `HttpResponse::NotFound()` to automatically send a 404 status code.
pub async fn api_not_found() -> impl Responder {
    let response = ApiResponse {
        status: "Error".to_string(),
        result: ResponseContent::Reason("API not found".to_string()),
    };

    HttpResponse::NotFound().json(response)
}

fn build_response(format: Option<&str>, response: &ApiResponse) -> impl Responder {
    format
        .map(|format| match format {
            "xml" => HttpResponse::Ok()
                .content_type("application/xml; charset=utf-8")
                .body(serde_xml_rs::to_string(response).unwrap()),
            _ => HttpResponse::Ok().json(response),
        })
        .unwrap_or(HttpResponse::Ok().json(response))
}

#[get("/posts")]
pub async fn api_index(query: web::Query<Queries>) -> impl Responder {
    let posts = get_all();

    let format = query.format.as_deref();
    let response = ApiResponse {
        status: "OK".to_string(),
        result: ResponseContent::Items(posts),
    };
    build_response(format, &response)
}

#[get("/posts/{id}")]
pub async fn api_show(id: web::Path<i32>, query: web::Query<Queries>) -> impl Responder {
    let id = id.into_inner();
    let post = get(id);

    let format = query.format.as_deref();
    let response = ApiResponse {
        status: "OK".to_string(),
        result: ResponseContent::Item(post),
    };
    build_response(format, &response)
}

#[post("/posts/create")]
pub async fn api_create(params: web::Json<Message>) -> impl Responder {
    let Message {
        sender, content, ..
    } = params.0;
    let now = Local::now();
    let posted = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let mut message = Message {
        id: 0,
        posted,
        sender,
        content,
    };
    message = data::create(message);

    let format = Some("json");
    let response = ApiResponse {
        status: "OK".to_string(),
        result: ResponseContent::Item(message),
    };
    build_response(format, &response)
}

#[put("/posts/update")]
pub async fn api_update(params: web::Json<Message>) -> impl Responder {
    let Message {
        id,
        posted,
        sender,
        content,
    } = params.0;
    let message = Message {
        id,
        posted,
        sender,
        content,
    };
    data::update(&message);

    let format = Some("json");
    let response = ApiResponse {
        status: "OK".to_string(),
        result: ResponseContent::Item(message),
    };
    build_response(format, &response)
}

#[delete("/posts/{id}/delete")]
pub async fn api_delete(id: web::Path<i32>, query: web::Query<Queries>) -> impl Responder {
    let id = id.into_inner();
    data::remove(id);

    let format = query.format.as_deref();
    let response = ApiResponse {
        status: "OK".to_string(),
        result: ResponseContent::None,
    };
    build_response(format, &response)
}
