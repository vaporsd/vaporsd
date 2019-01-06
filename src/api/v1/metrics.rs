use actix_web::{Scope, HttpRequest, HttpResponse};

pub fn root<S: 'static>(_: HttpRequest<S>) -> HttpResponse {
    HttpResponse::Ok().body("metrics")
}

pub fn main<S: 'static>(scope: Scope<S>) -> Scope<S> {
    scope
}
