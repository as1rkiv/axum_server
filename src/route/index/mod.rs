use crate::common::trace::X_REQUEST_ID;
use axum::{
    body::Body,
    http::{HeaderMap, Request},
    response::Html,
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/", get(index))
}

// 主页
async fn index(req: Request<Body>) -> Html<String> {
    let headers: &HeaderMap = req.headers();
    let request_id = match headers.get(X_REQUEST_ID) {
        Some(val) => val.to_str().unwrap_or("Client"),
        None => "Anonymous",
    };

    Html(format!(
        r#"
      <!DOCTYPE html>
      <html>
          <head>
               <meta charset="utf-8">
          </head>
          <body>
              <h1 style="text-align: center; margin-top: 64px;">Hi, {}</h1>
          </body>
      </html>
      "#,
        request_id
    ))
}
