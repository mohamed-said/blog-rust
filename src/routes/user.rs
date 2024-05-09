use axum::{extract::State, routing::post, Json, Router};
use bson::oid::ObjectId;

use crate::models::user::{AddUserRequest, UserController};

use crate::error::user_error::Result;
pub fn routes(state: UserController) -> Router {
    Router::new()
        .route("/create-user", post(create_user))
        .with_state(state)
}

pub async fn create_user(
    State(state): State<UserController>,
    Json(req): Json<AddUserRequest>,
) -> Result<Json<ObjectId>> {
    println!("->> {:12} - create_user", "HANDLER");

    let added_object_id = state.create_user(req).await?;

    Ok(Json(added_object_id))
}
