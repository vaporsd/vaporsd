use actix_web::{App, HttpRequest, HttpResponse, Result, FromRequest};
use actix_web::middleware::{Middleware, Started};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};

mod response;
mod v1;

fn checkpermission(r: &mut HttpRequest) -> bool {
    let auth = BearerAuth::from_request(&r, &Config::default()).unwrap();

    auth.token() == "lol"
}

pub fn root() -> App {
    App::new()
        .scope("/v1", v1::root) // Import the v1 API.
        .default_resource(|r|
            /* Display a proper 404 json response */
            r.f(response::default)
        )
    // <--
}
