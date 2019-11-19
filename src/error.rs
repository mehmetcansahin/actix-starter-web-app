use crate::app::models::response::ResponseBody;
use actix_web::{
    dev,
    http::{header::CONTENT_TYPE, HeaderValue, StatusCode},
    middleware::errhandlers::ErrorHandlerResponse,
    HttpResponse, Result,
};

pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(),
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}

pub fn render_400<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut()
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("Error"));
    let error =
        ServiceError::new(StatusCode::BAD_REQUEST, "400 BAD REQUEST".to_string()).response();
    Ok(ErrorHandlerResponse::Response(
        res.into_response(error.into_body()),
    ))
}
