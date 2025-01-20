use crate::proto::algebraic::algebraic_client::AlgebraicClient;
pub use crate::proto::algebraic::{
    ExponentMessage, FactorialMessage, FloatResponse, IntegerResponse,
};
use tonic::transport::Channel;
use tonic::Request;

mod proto;

#[derive(Clone)]
pub struct AlgebraicGrpcInterface {
    client: AlgebraicClient<Channel>,
}

impl AlgebraicGrpcInterface {
    pub async fn new(addr: &'static str, port: u16) -> Self {
        let channel = Channel::from_shared(format!("http://{addr}:{port}"))
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            client: AlgebraicClient::new(channel),
        }
    }

    pub async fn exponent(&mut self, message: ExponentMessage) -> Option<FloatResponse> {
        self.client
            .exponent(Request::new(message))
            .await
            .map(tonic::Response::into_inner)
            .ok()
    }

    pub async fn factorial(&mut self, message: FactorialMessage) -> Option<IntegerResponse> {
        self.client
            .factorial(Request::new(message))
            .await
            .map(tonic::Response::into_inner)
            .ok()
    }
}
