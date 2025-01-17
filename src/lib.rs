use crate::proto::algebraic::algebraic_client::AlgebraicClient;
use crate::proto::algebraic::{ExponentMessage, FactorialMessage};
use tonic::transport::Channel;
use tonic::Request;

mod proto;

pub struct AlgebraicClientImpl {
    pub addr: String,
    pub port: u16,
}

impl AlgebraicClientImpl {
    pub async fn exponent(&self, base: u64, exponent: u64) -> Option<u64> {
        let channel = Channel::from_shared(format!("http://{}:{}", self.addr, self.port))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let mut client = AlgebraicClient::new(channel);

        let exponent_message = ExponentMessage { base, exponent };

        client
            .exponent(Request::new(exponent_message))
            .await
            .map(|x| x.into_inner().value)
            .ok()
    }

    pub async fn factorial(&self, value: u64) -> Option<u64> {
        let channel = Channel::from_shared(format!("http://{}:{}", self.addr, self.port))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let mut client = AlgebraicClient::new(channel);

        let factorial_message = FactorialMessage { value };

        client
            .factorial(Request::new(factorial_message))
            .await
            .map(|x| x.into_inner().value)
            .ok()
    }
}
