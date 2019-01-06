use actix_web::{HttpRequest, HttpResponse};

pub fn default<S: 'static>(r: HttpRequest<S>) -> HttpResponse {
    /* Return a NotFound response */
    HttpResponse::NotFound().json(
        json!({
            "errors": [
                {}
            ]
        })
    )
}
