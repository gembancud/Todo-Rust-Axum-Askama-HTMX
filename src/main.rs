use api::utils::{get_server_address, get_tcp_listener, setup_environment, setup_tracing};

mod api;

#[tokio::main]
async fn main() {
    setup_environment();
    setup_tracing();

    let app = api::create_api();
    let addr = get_server_address();

    tracing::info!("Starting server at http://{}", addr);

    let listener = get_tcp_listener(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
