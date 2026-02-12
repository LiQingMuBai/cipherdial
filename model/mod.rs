use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct PhoneVerification {
    pub id: String,

    //#[validate(phone(message = "手机号码格式不正确"))]
    pub phone: String,

    #[validate(length(min = 2, max = 100, message = "用户名长度必须在2-100字符之间"))]
    pub username: String,

    #[validate(length(min = 4, max = 8, message = "验证码长度必须在4-8字符之间"))]
    pub verification_code: String,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PhoneVerification {
    pub fn new(phone: String, username: String, verification_code: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            phone,
            username,
            verification_code,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateVerificationRequest {
    //#[validate(phone(message = "手机号码格式不正确"))]
    pub phone: String,

    #[validate(length(min = 2, max = 100, message = "用户名长度必须在2-100字符之间"))]
    pub username: String,

    #[validate(length(min = 4, max = 8, message = "验证码长度必须在4-8字符之间"))]
    pub verification_code: String,
}

// 新增：根据用户名获取手机号的请求
#[derive(Debug, Deserialize, Validate)]
pub struct GetPhoneRequest {
    #[validate(length(min = 2, max = 100, message = "用户名长度必须在2-100字符之间"))]
    pub username: String,
}

// 新增：手机号响应
#[derive(Debug, Serialize)]
pub struct PhoneResponse {
    pub phone: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: "操作成功".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

// 为 () 类型提供专门的空响应方法
impl ApiResponse<()> {
    #[allow(dead_code)]
    pub fn success_empty() -> Self {
        Self {
            success: true,
            message: "操作成功".to_string(),
            data: None,
        }
    }

    pub fn error_empty(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}