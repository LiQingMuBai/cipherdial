use axum::routing::post;
use axum::Router;
use crate::AppState;

pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/verify", post(crate::controllers::verification_controller::verify_handler))
}