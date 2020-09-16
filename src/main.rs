mod api;
mod config;
mod errors;
mod routes;
mod server;
mod service;
mod store;
mod util;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate version;

extern crate tracing;

use crate::config::config::Config;
use crate::server::state::AppState;
use crate::store::model::store::Store;

use actix_http::KeepAlive;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use gumdrop::Options;
use hocon::Hocon;
use std::io;
use std::sync::Arc;
use tracing::{info, Level};

#[derive(Debug, Options)]
struct ArgsOpts {
    #[options(help = "environment", short = "e")]
    env: Option<String>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let options = ArgsOpts::parse_args_default_or_exit();

    let config = Config::get_config(options.env).unwrap();

    let _subscriber = tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        //.compact()
        .with_max_level(config.log_config.log_level.parse::<Level>().unwrap())
        // completes the builder and sets the constructed `Subscriber` as the default.
        .init();

    let store = Store::new(&config).await.unwrap();
    let state = Arc::new(AppState { store: store });

    info!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    let payload_size = Hocon::String(config.server.payload_size)
        .as_bytes()
        .unwrap() as usize;

    info!("Set payload size in: ${:?}", payload_size);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new(util::log::ACTIX_LOG_FORMAT))
            .wrap(middleware::Compress::default())
            .data(state.clone())
            .data(
                web::JsonConfig::default()
                    .limit(payload_size)
                    .error_handler(|err, _req| {
                        // create custom error response
                        error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                            .into()
                    }),
            )
            .service(
                web::resource("/actix-example/test")
                    .route(web::post().to(routes::example::post))
                    .route(web::get().to(routes::example::query)),
            )
            .route(
                "/actix-example/health-check",
                web::get().to(routes::health_check::health_check),
            )
            .route(
                "/actix-example/version",
                web::get().to(routes::health_check::version),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    //.keep_alive(KeepAlive::Timeout(config.server.keep_alive))
    .keep_alive(KeepAlive::Os)
    .system_exit()
    .run()
    .await
}
