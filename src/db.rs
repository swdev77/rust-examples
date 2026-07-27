use sqlx_firebirdsql::{FirebirdConnectOptions, FirebirdPool};
use crate::config::{FbConfig};

pub async fn get_fb_pool(fb_config: FbConfig) -> FirebirdPool{
    let connect_options = FirebirdConnectOptions::new()
        .host(&fb_config.host)
        .port(fb_config.port)
        .database(&fb_config.file)
        .username(&fb_config.user)
        .password(&fb_config.password);

    FirebirdPool::connect_with(connect_options)
        .await
        .expect("Not not connect to Firebird")
}