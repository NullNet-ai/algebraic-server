use crate::proto::algebraic::algebraic_server::Algebraic;
use crate::proto::algebraic::{ExponentMessage, FactorialMessage, FloatResponse, IntegerResponse};
use tonic::{Request, Response, Status};

pub struct AlgebraicImpl;

#[tonic::async_trait]
impl Algebraic for AlgebraicImpl {
    async fn exponent(
        &self,
        request: Request<ExponentMessage>,
    ) -> Result<Response<FloatResponse>, Status> {
        let ExponentMessage { base, exponent } = request.into_inner();

        let mut res = 1.0;
        for _ in 0..exponent {
            res = multiply::multiply(res, base);
        }

        let response = FloatResponse { value: res };
        Ok(Response::new(response))
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    async fn factorial(
        &self,
        request: Request<FactorialMessage>,
    ) -> Result<Response<IntegerResponse>, Status> {
        let FactorialMessage { value } = request.into_inner();

        let mut res = 1;
        let mut cnt = 1;
        loop {
            if cnt >= value {
                break;
            }

            cnt += 1;
            res = multiply::multiply(res as f32, cnt as f32) as u64;
        }

        let response = IntegerResponse { value: res };
        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_exponent() {
        let algebraic_impl = AlgebraicImpl;
        let message = ExponentMessage {
            base: 2.0,
            exponent: 3,
        };
        let request = Request::new(message);

        let response = algebraic_impl.exponent(request).await.unwrap();
        assert_eq!(response.into_inner().value, 8.0);
    }

    #[tokio::test]
    async fn test_factorial() {
        let algebraic_impl = AlgebraicImpl;
        let message = FactorialMessage { value: 10 };
        let request = Request::new(message);

        let response = algebraic_impl.factorial(request).await.unwrap();
        assert_eq!(response.into_inner().value, 3628800);
    }
}
