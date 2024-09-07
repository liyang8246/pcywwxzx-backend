use api::{get_verifycode, issue::*};
use model::{AppState, State};
use reqwest::Method;
use salvo::{conn::native_tls::NativeTlsConfig, cors::Cors, prelude::*};
use sqlx::sqlite::SqlitePoolOptions;
use std::fs;
use std::{
    collections::HashMap,
    env,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::Mutex, time::sleep};

mod api;
mod model;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let mxnzp_appid = env::var("MXNZP_APPID")?;
    let mxnzp_secret = env::var("MXNZP_SECRET")?;
    let manager_passwd = env::var("MANAGER_PASSWD")?;
    let pkcs12_passwd = env::var("PKCS12_PASSWD")?;
    let app_state: State = Arc::new(Mutex::new(AppState {
        db_pool: SqlitePoolOptions::new()
            .max_connections(4)
            .connect(&db_url)
            .await
            .expect("数据库连接失败"),
        mxnzp_appid,
        mxnzp_secret,
        manager_passwd,
        verifycode: HashMap::new(),
    }));

    let state_clone = Arc::clone(&app_state);
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;
            let mut state = state_clone.lock().await;
            state
                .verifycode
                .retain(|_, (_, i)| *i + Duration::from_secs(300) > Instant::now());
        }
    });

    let cors = Cors::new()
        .allow_origin("https://www.pcywwxzx.top")
        .allow_methods(vec![
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .into_handler();

    let acceptor = TcpListener::new("0.0.0.0:5800")
        .native_tls(async_stream::stream! {
                loop {
                    yield load_config(&pkcs12_passwd);
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
        })
        .bind()
        .await;

    let route = Router::new()
        .get(hello)
        .hoop(affix_state::inject(app_state))
        .hoop(cors)
        .push(Router::with_path("api")
                .push(Router::with_path("verifycode")
                    .get(get_verifycode))
                .push(Router::with_path("issue")
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

fn load_config(pkcs12_passwd: &str) -> NativeTlsConfig {
    let pkcs12 = fs::read("data/certs/identity.p12").expect("unable to read pkcs12");
    NativeTlsConfig::new()
        .pkcs12(pkcs12)
        .password(pkcs12_passwd)
}
