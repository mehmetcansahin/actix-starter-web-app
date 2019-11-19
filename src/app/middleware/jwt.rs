use crate::app::models::{response::ResponseBody, user::User};
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures::{
    future::{ok, Either, FutureResult},
    Poll,
};

pub struct Token;

impl<S, B> Transform<S> for Token
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TokenMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TokenMiddleware { service })
    }
}
pub struct TokenMiddleware<S> {
    service: S,
}

impl<S, B> Service for TokenMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let except_routes = vec!["/api/auth/register", "/api/auth/login"];
        for route in except_routes.iter() {
            if req.path().starts_with(route) {
                return Either::A(self.service.call(req));
            }
        }

        let headers = req.headers();
        match User::decode_token_from_headers(headers) {
            Ok(_auth_email) => Either::A(self.service.call(req)),
            Err(err) => Either::B(ok(req.into_response(
                HttpResponse::Unauthorized()
                    .json(ResponseBody::new("JWT TOKEN ERROR", err.to_string()))
                    .into_body(),
            ))),
        }
    }
}
