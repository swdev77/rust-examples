use sqlx_firebirdsql::FirebirdConnectOptions;
use crate::config::{FbConfig};

pub fn get_fb_connect_options(fb_config: FbConfig) -> FirebirdConnectOptions {
    // Load Firebird configuration from environment variables
    
    // Build the connection options from the loaded configuration
    FirebirdConnectOptions::new()
        .host(&fb_config.host)
        .port(fb_config.port)
        .database(&fb_config.file)
        .username(&fb_config.user)
        .password(&fb_config.password)
}