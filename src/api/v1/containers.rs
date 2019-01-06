use actix_web::{Scope, HttpRequest, HttpResponse};

pub fn root<S: 'static>(_: HttpRequest<S>) -> HttpResponse {
    HttpResponse::Ok().body("list")
}

pub fn main<S: 'static>(scope: Scope<S>) -> Scope<S> {
    scope
        .resource("/add", |r| r.f(|_|HttpResponse::Ok().body("add")))
        .resource("/{uuid}/remove", |r| r.f(|_|HttpResponse::Ok().body("remove")))
        .resource("/{uuid}/assignto", |r| r.f(|_|HttpResponse::Ok().body("assignto")))
}
