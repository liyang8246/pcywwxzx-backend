use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tokio::sync::Mutex;


pub type State = Arc<Mutex<AppState>>;
#[allow(dead_code)]
pub struct AppState {
    pub db_pool: SqlitePool,
    pub mxnzp_appid: String,
    pub mxnzp_secret: String,
    pub manager_passwd: String,
    pub verifycode: HashMap<String, (String, Instant)>,
}
