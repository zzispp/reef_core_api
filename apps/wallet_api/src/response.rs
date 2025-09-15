use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            code: 500,
            message,
            data: None,
        }
    }
}

// 自定义错误类型，包装anyhow::Error
#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_message = format!("{:#}", self.0);
        HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error_message))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError(error)
    }
}

// 定义一个类型别名，让API函数更简洁
pub type ApiResult = std::result::Result<HttpResponse, AppError>;

// 辅助函数，将成功的数据转换为标准响应
pub fn success_response<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success(data))
}
