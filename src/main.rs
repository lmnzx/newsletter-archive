use {
    axum::{
        body::Bytes,
        handler::Handler,
        http::Request,
        middleware::{self},
        response::Response,
        routing::get,
        Router,
    },
    std::{future::ready, net::SocketAddr, time::Duration},
    tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer},
    tracing::Span,
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

use newsletter::graceful_shutdown::shutdown_signal;
use newsletter::metrics::{setup_metrics_recorder, track_metrics};
use newsletter::routes::{global_404, health_check, subscriptions};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "pscale=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let recorder_handle = setup_metrics_recorder();

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", get(subscriptions))
        .fallback(global_404.into_service())
        .route("/metrics", get(move || ready(recorder_handle.render())))
        .route_layer(middleware::from_fn(track_metrics))
        .layer(TraceLayer::new_for_http())
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, _span: &Span| {
                    tracing::debug!("started {} {}", request.method(), request.uri().path())
                })
                .on_response(|_response: &Response, latency: Duration, _span: &Span| {
                    tracing::debug!("response generated in {:?}", latency)
                })
                .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
                    tracing::debug!("sending {} bytes", chunk.len())
                })
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("something went wrong {:#?}", error)
                    },
                ),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listing on address {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
