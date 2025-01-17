use crate::proto::algebraic::algebraic_server::AlgebraicServer;
use crate::r#impl::AlgebraicImpl;
use std::net::ToSocketAddrs;
use tonic::transport::Server;

mod r#impl;
mod proto;

#[tokio::main]
async fn main() {
    let addr = "localhost:50051"
        .to_string()
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    Server::builder()
        .add_service(AlgebraicServer::new(AlgebraicImpl))
        .serve(addr)
        .await
        .unwrap();
}
