use reqwest::{header::*, Method};
use salvo::prelude::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::time;
use tracing::{error, info};

#[handler]
pub async fn cors_middleware(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    res.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    res.headers_mut().insert(
        ACCESS_CONTROL_ALLOW_METHODS,
        "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap(),
    );
    res.headers_mut()
        .insert(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization".parse().unwrap());
    if req.method() == Method::OPTIONS {
        res.status_code = Some(StatusCode::NO_CONTENT);
        ctrl.skip_rest();
    }
    ctrl.call_next(req, depot, res).await;
}

pub async fn connect_db(db_url: &str) -> Pool<Postgres> {
    loop {
        match PgPoolOptions::new().max_connections(8).connect(db_url).await {
            Ok(pool) => {
                info!("Connected to database with url: {}", db_url);
                return pool;
            }
            Err(e) => {
                error!("Failed to connect to database: {:?}", e);
                time::sleep(time::Duration::from_secs(5)).await;
            }
        }
    }
}
