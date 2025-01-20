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
