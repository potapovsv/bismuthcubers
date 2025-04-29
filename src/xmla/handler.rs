use actix_web::{web, HttpRequest, HttpResponse};
use std::str;
use crate::xmla::parser;

// Обработчик XMLA-запросов
pub async fn xmla_handler(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let content_type = req.headers().get("Content-Type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("");

    if content_type.contains("text/xml") || content_type.contains("application/soap+xml") {
        match parser::parse_xmla(&body) {
            Ok(request_type) => {
                // TODO: добавить реальную логику Discover / Execute
                HttpResponse::Ok()
                    .content_type("text/xml; charset=utf-8")
                    .body(format!(
                        "<return><status>OK</status><type>{}</type></return>",
                        request_type
                    ))
            },
            Err(e) => HttpResponse::BadRequest().body(e),
        }
    } else {
        HttpResponse::UnsupportedMediaType().body("Unsupported Content-Type")
    }
}