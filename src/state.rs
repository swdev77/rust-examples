use sqlx_firebirdsql::FirebirdConnectOptions;

#[derive(Clone)]
pub struct AppState {
    pub connect_options: FirebirdConnectOptions,
}