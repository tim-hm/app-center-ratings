use std::sync::Arc;
use std::{net::SocketAddr, time::Duration};

use tonic::transport::Server;
use tower::ServiceBuilder;
use tracing::info;

use crate::app::interfaces::authentication::authentication;
use crate::app::interfaces::interfaces::{
    build_private_servers, build_public_servers, build_reflection_service,
};
use crate::app::interfaces::middleware::ContextMiddlewareLayer;
use crate::utils;

use super::infrastructure::Infrastructure;

pub async fn build_and_run() {
    let layer = ServiceBuilder::new()
        .timeout(Duration::from_secs(30))
        .layer(ContextMiddlewareLayer::default())
        .layer(tonic::service::interceptor(authentication))
        .into_inner();

    let infra = Infrastructure::new().await;
    let infra = Arc::new(infra);

    let server = Server::builder()
        .layer(layer)
        .add_service(build_reflection_service());

    let server = build_public_servers(server);
    let server = build_private_servers(server);

    let socket: SocketAddr = utils::env::get_socket().parse().unwrap();
    info!("Binding to {socket}");
    server.serve(socket).await.unwrap();
}
