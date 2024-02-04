use listenfd::ListenFd;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub fn setup_environment() {
    dotenvy::dotenv().ok();
}

pub fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "laced=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn get_server_address() -> String {
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    format!("{}:{}", host, port)
}

pub async fn get_tcp_listener(addr: &str) -> std::io::Result<TcpListener> {
    let mut listenfd = ListenFd::from_env();
    match listenfd.take_tcp_listener(0)? {
        Some(listener) => Ok(TcpListener::from_std(listener)?),
        None => TcpListener::bind(addr).await,
    }
}
