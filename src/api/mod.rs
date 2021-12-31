mod handlers;

use axum::{
    routing::{get, post},
    Router,
};

pub(crate) fn routes() -> Router {
    // Might want to play with GraphQL later or simply do breaking changes to the api, so we use `v1` path
    let api_v1_routes = Router::new()
        .route("/login", post(handlers::handle_login))
        .route(
            "/accounts",
            get(handlers::get_accounts).post(handlers::create_account),
        )
        .route("/accounts/:id", get(handlers::get_specific_account));

    let api_v1_nest = Router::new().nest("/v1", api_v1_routes);
    Router::new().nest("/api", api_v1_nest)
}
