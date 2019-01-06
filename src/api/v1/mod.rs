use actix_web::{Scope};
use api::response;

mod users;
mod containers;
mod metrics;

pub fn root<S: 'static>(scope: Scope<S>) -> Scope<S> {
    scope
        // Append /users to the API
        .resource("/users", |r| r.f(users::root))
        .nested("/users", users::main)

        // Append /containers to the API
        .resource("/containers", |r| r.f(containers::root))
        .nested("/containers", containers::main)

        // Append /metrics to the API
        .resource("/metrics", |r| r.f(metrics::root))
        .nested("/metrics", metrics::main)

        .default_resource(|r|
            /* Display a proper 404 json response */
            r.f(response::default)
        )
    // <--
}
