use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tokio::sync::RwLock;

pub type State = Arc<RwLock<AppState>>;
#[allow(dead_code)]
pub struct AppState {
    pub version:        String,
    pub db_pool:        PgPool,
    pub mxnzp_appid:    String,
    pub mxnzp_secret:   String,
    pub manager_passwd: String,
    pub verifycode:     HashMap<String, (String, Instant)>,
}
