mod api;
mod crud;
mod error;
mod extract;
mod html_template;
pub(crate) mod models;
mod requests;
mod utils;
mod views;

use std::{env, io, net::SocketAddr};

use {
    anyhow::Context,
    axum::{
        handler::Handler, http::StatusCode, response::IntoResponse, routing::get_service,
        AddExtensionLayer, Router,
    },
    dotenv::dotenv,
    tower_cookies::CookieManagerLayer,
    tower_http::{services::ServeDir, trace::TraceLayer},
    tracing::dispatcher::SetGlobalDefaultError,
    tracing_subscriber::EnvFilter,
};

pub(crate) use error::{CommonError, Error};

use crate::{html_template::HtmlTemplate, models::user::UserClaims};

fn setup_logging() -> Result<(), SetGlobalDefaultError> {
    let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| String::from("DEBUG,hyper=INFO"));

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
        eprintln!("[WARNING]: Failed to add default admin user: {:#}", e);
    }

    let secret = env::var("SECRET").context("Expected `SECRET` env variable")?;
    utils::set_secret(&secret);

    let port = env::var("PORT")
        .context("Missing env variable `PORT`")?
        .parse::<u16>()
        .context("`PORT` env variable is not valid")?;

    let static_files = Router::new().nest(
        "/static",
        get_service(ServeDir::new("static")).handle_error(|err: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", err),
            )
        }),
    );

    let api = api::routes();
    let views = views::routes();

    let app = static_files
        .merge(api)
        .merge(views)
        .fallback(handle_404.into_service())
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .layer(AddExtensionLayer::new(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Failed to run axum::Server")
}

async fn handle_404(user: Option<UserClaims>) -> impl IntoResponse {
    let view = if let Some(user) = user {
        views::views_404::View404::new(Some(&user.username))
    } else {
        views::views_404::View404::new(None)
    };

    HtmlTemplate(view)
}

async fn shutdown_signal() {
    #[cfg(unix)]
    async fn terminate() -> io::Result<()> {
        use tokio::signal::unix::SignalKind;

        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }
    #[cfg(not(unix))]
    async fn terminate() -> io::Result<()> {
        unimplemented!("Implement this for non-unix");
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    tracing::info!("signal received, starting graceful shutdown")
}
