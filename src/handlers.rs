
use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use sqlx::Row;
use crate::{models::User, state::{FbState}};

pub async fn get_users(
    State(state): State<FbState>,
) -> Result<Json<Vec<User>>, StatusCode> {

    let user_qry = "select id, title, lgn from users";

    let rows = sqlx::query(user_qry)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let users = rows
        .iter()
        .map(|row| { 
            User { 
                id: row.get("ID"), 
                title: row.get("TITLE"), 
                username: row.get("LGN")
            } 
        })
        .collect();

    Ok(Json(users))
}