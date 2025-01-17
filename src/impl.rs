use crate::proto::algebraic::algebraic_server::Algebraic;
use crate::proto::algebraic::{ExponentMessage, FactorialMessage, IntegerResponse};
use tonic::{Request, Response, Status};

pub struct AlgebraicImpl;

#[tonic::async_trait]
impl Algebraic for AlgebraicImpl {
    async fn exponent(
        &self,
        request: Request<ExponentMessage>,
    ) -> Result<Response<IntegerResponse>, Status> {
        let ExponentMessage { base, exponent } = request.into_inner();

        let mut res = 1;
        for _ in 0..exponent {
            res = multiply::multiply(res, base);
        }

        let response = IntegerResponse { value: res };
        Ok(Response::new(response))
    }

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
            res = multiply::multiply(res, cnt);
        }

        let response = IntegerResponse { value: res };
        Ok(Response::new(response))
    }
}
