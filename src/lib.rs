use crate::proto::algebraic::algebraic_client::AlgebraicClient;
pub use crate::proto::algebraic::{
    ExponentMessage, FactorialMessage, FloatResponse, IntegerResponse,
};
use tonic::transport::Channel;
use tonic::Request;

mod proto;

pub struct AlgebraicClientImpl {
    pub addr: String,
    pub port: u16,
}

impl AlgebraicClientImpl {
    pub async fn exponent(&self, message: ExponentMessage) -> Option<FloatResponse> {
        let channel = Channel::from_shared(format!("http://{}:{}", self.addr, self.port))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let mut client = AlgebraicClient::new(channel);

        client
            .exponent(Request::new(message))
            .await
            .map(tonic::Response::into_inner)
            .ok()
    }

    pub async fn factorial(&self, message: FactorialMessage) -> Option<IntegerResponse> {
        let channel = Channel::from_shared(format!("http://{}:{}", self.addr, self.port))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let mut client = AlgebraicClient::new(channel);

        client
            .factorial(Request::new(message))
            .await
            .map(tonic::Response::into_inner)
            .ok()
    }
}
