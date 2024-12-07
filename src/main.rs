use api::{issue::*, *};
use model::{AppState, State};
use reqwest::Method;
use salvo::{cors::Cors, prelude::*};
use std::{
    collections::HashMap,
    env,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::Mutex, time::sleep};
use utils::*;

mod api;
mod model;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    // load env
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let mxnzp_appid = env::var("MXNZP_APPID")?;
    let mxnzp_secret = env::var("MXNZP_SECRET")?;
    let manager_passwd = env::var("MANAGER_PASSWD")?;
    // init app_state
    let app_state: State = Arc::new(Mutex::new(AppState {
        version: env!("CARGO_PKG_VERSION").to_string(),
        db_pool: connect_db(&db_url).await,
        mxnzp_appid,
        mxnzp_secret,
        manager_passwd,
        verifycode: HashMap::new(),
    }));
    // verifycode clean up
    let state_clone = Arc::clone(&app_state);
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;
            let mut state = state_clone.lock().await;
            state
                .verifycode
                .retain(|_, (_, i)| *i + Duration::from_secs(300) > Instant::now());
        }
    });
    // init cors
    let cors = Cors::new()
        .allow_origin([
            "https://www.pcywwxzx.top",
            "http://localhost:5173",
            "http://192.168.1.15:5173",
        ])
        .allow_methods(vec![Method::GET, Method::PUT, Method::POST, Method::DELETE, Method::OPTIONS])
        .into_handler();

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    let route = Router::new()
        .get(hello)
        .hoop(cors_middleware)
        .hoop(affix_state::inject(app_state))
        .hoop(cors)
        .push(
            Router::with_path("api")
                .push(Router::with_path("verifycode").get(get_verifycode))
                .push(Router::with_path("issue_num").get(get_issue_num))
                .push(Router::with_path("date_num").get(get_date_num))
                .push(Router::with_path("version").get(get_version))
                .push(
                    Router::with_path("issue")
                        .put(add_issue)
                        .options(add_issue)
                        .get(view_issue)
                        .post(toggle_issue)
                        .delete(del_issue),
                ),
        );
    Server::new(acceptor).serve(route).await;
    Ok(())
}

#[handler]
async fn hello(res: &mut Response) {
    res.render("welcome to pcywwxzx backend :)");
}
