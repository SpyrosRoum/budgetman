mod views;

use std::{convert::Infallible, env};

use {
    anyhow::Context,
    dotenv::dotenv,
    tracing::{debug, dispatcher::SetGlobalDefaultError},
    tracing_subscriber::EnvFilter,
    warp::{http::Uri, Filter, Rejection, Reply},
};

fn setup_logging() -> Result<(), SetGlobalDefaultError> {
    let app_name = concat!(env!("CARGO_PKG_NAME"), "_", env!("CARGO_PKG_VERSION"));
    debug!("Init Logger {}", app_name);
    let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| String::from("DEBUG"));

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(rust_log))
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    if let Err(e) = setup_logging() {
        eprintln!("[WARNING]: Failed to setup logging: {}", e);
    }

    let db = crud::create_db().await?;
    if let Err(e) = crud::add_default_user(&db).await {
        eprint!("Failed to add default admin user: {:#}", e);
    }

    let secret = env::var("SECRET").context("Expected `SECRET` env variable")?;
    common::set_secret(&secret);

    let api = api::routes(&db);
    let views = views::routes(&db);
    let admin_lte = warp::path("static").and(warp::fs::dir("static"));

    let routes = admin_lte.or(views).or(api).recover(handle_rejection);

    let routes_with_logs = routes.with(warp::trace::trace(|info| {
        tracing::info_span!(
            "request",
            method = %info.method(),
            path = %info.path()
        )
    }));
    warp::serve(routes_with_logs)
        .run(([0, 0, 0, 0], 8000))
        .await;

    Ok(())
}

/// Catch any unhandled routes (404) or internal errors (505)
/// All internal errors must have been caught earlier but we check it here as well just in case
async fn handle_rejection(r: Rejection) -> Result<impl Reply, Infallible> {
    let uri = if r.is_not_found() {
        Uri::from_static("/404")
    } else {
        Uri::from_static("/500")
    };

    Ok(warp::redirect(uri))
}
