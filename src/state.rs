use sqlx_firebirdsql::FirebirdPool;

#[derive(Clone)]
pub struct FbState {
    pub pool: FirebirdPool,
}