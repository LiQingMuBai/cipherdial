use axum::{
    Json,
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use validator::Validate;

use crate::model::{
    ApiResponse, CreateVerificationRequest, PhoneVerification,
    GetPhoneRequest, PhoneResponse
};
use crate::service::VerificationService;

pub struct AppState {
    pub verification_service: VerificationService,
}

// 修改：使用 create_or_update_verification 替代原有的 create_verification
pub async fn create_or_update_verification(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateVerificationRequest>,
) -> impl IntoResponse {
    // 验证请求数据
    if let Err(e) = payload.validate() {
        let errors: Vec<String> = e.field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter()
                    .filter_map(|e| e.message.as_ref())
                    .map(|msg| msg.to_string())
                    .collect::<Vec<_>>()
            })
            .collect();

        let response = ApiResponse::<()>::error_empty(&errors.join(", "));
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    match state.verification_service.create_or_update_verification(payload).await {
        Ok(verification) => {
            // 判断是新增还是更新
            let is_update = verification.created_at != verification.updated_at;
            let message = if is_update {
                "验证码更新成功"
            } else {
                "验证码创建成功"
            };

            let response = ApiResponse {
                success: true,
                message: message.to_string(),
                data: Some(verification),
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            let response = ApiResponse::<()>::error_empty(&format!("数据库错误: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// 保留原有的创建方法（如果需要）
#[allow(dead_code)]
pub async fn create_verification(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateVerificationRequest>,
) -> impl IntoResponse {
    // 验证请求数据
    if let Err(e) = payload.validate() {
        let errors: Vec<String> = e.field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter()
                    .filter_map(|e| e.message.as_ref())
                    .map(|msg| msg.to_string())
                    .collect::<Vec<_>>()
            })
            .collect();

        let response = ApiResponse::<()>::error_empty(&errors.join(", "));
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    match state.verification_service.create_verification(payload).await {
        Ok(verification) => {
            let response = ApiResponse::success(verification);
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            let response = ApiResponse::<()>::error_empty(&format!("数据库错误: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

pub async fn get_verifications(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.verification_service.get_verifications().await {
        Ok(verifications) => {
            let response = ApiResponse::success(verifications);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            let response = ApiResponse::<Vec<PhoneVerification>>::error(&format!("数据库错误: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

pub async fn get_phone_by_username_path(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    // 验证用户名长度
    if username.len() < 2 || username.len() > 100 {
        let response = ApiResponse::<()>::error_empty("用户名长度必须在2-100字符之间");
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    match state.verification_service.get_phone_by_username(&username).await {
        Ok(Some(phone)) => {
            let response = ApiResponse::success(PhoneResponse {
                phone,
                username,
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response = ApiResponse::<()>::error_empty("未找到该用户的手机号码");
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(e) => {
            let response = ApiResponse::<()>::error_empty(&format!("数据库错误: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

pub async fn get_phone_by_username_json(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GetPhoneRequest>,
) -> impl IntoResponse {
    // 验证请求数据
    if let Err(e) = payload.validate() {
        let errors: Vec<String> = e.field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter()
                    .filter_map(|e| e.message.as_ref())
                    .map(|msg| msg.to_string())
                    .collect::<Vec<_>>()
            })
            .collect();

        let response = ApiResponse::<()>::error_empty(&errors.join(", "));
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    match state.verification_service.get_phone_by_username(&payload.username).await {
        Ok(Some(phone)) => {
            let response = ApiResponse::success(PhoneResponse {
                phone,
                username: payload.username,
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response = ApiResponse::<()>::error_empty("未找到该用户的手机号码");
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(e) => {
            let response = ApiResponse::<()>::error_empty(&format!("数据库错误: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

pub async fn get_verifications_by_username(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    // 验证用户名长度
    if username.len() < 2 || username.len() > 100 {
        let response = ApiResponse::<()>::error_empty("用户名长度必须在2-100字符之间");
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    match state.verification_service.get_verifications_by_username(&username).await {
        Ok(verifications) => {
            if verifications.is_empty() {
                let response = ApiResponse::<Vec<PhoneVerification>>::error("未找到该用户的任何记录");
                (StatusCode::NOT_FOUND, Json(response)).into_response()
            } else {
                let response = ApiResponse::success(verifications);
                (StatusCode::OK, Json(response)).into_response()
            }
        }
        Err(e) => {
            let response = ApiResponse::<Vec<PhoneVerification>>::error(&format!("数据库错误: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// 健康检查接口
pub async fn health_check() -> impl IntoResponse {
    let response = ApiResponse::success(serde_json::json!({ "status": "healthy" }));
    (StatusCode::OK, Json(response))
}