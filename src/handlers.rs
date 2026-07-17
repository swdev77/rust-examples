
use sqlx::{ConnectOptions, Row};
use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};

use crate::{models::User, state::AppState};

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, StatusCode> {

    let mut conn = state.connect_options
        .connect()
        .await
        .unwrap();

    // let rows = Executor::fetch_all(
    //     &mut conn, 
    //     "select id from users")
    //     .await
    //     .expect("fetching users failed");

    let rows = sqlx::query("select id, title, lgn from users")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    
    let users = rows
        .into_iter()
        .map(|row| { 
            let id = row.try_get("ID").unwrap();
            let title: String = row.try_get("TITLE").unwrap();
            let lgn: String = row.try_get("LGN").unwrap();
            
            User { id: id, title: title, username: lgn} 
        })
        .collect();

    Ok(Json(users))
}