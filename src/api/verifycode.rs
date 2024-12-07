use crate::model::{AppResult, State};
use salvo::prelude::*;
use std::time::{Duration, Instant};
use tokio::time;

#[handler]
pub async fn get_verifycode(depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let mut appstate = depot.obtain::<State>().expect("get appstate fail").write().await;
    let app_id = appstate.mxnzp_appid.clone();
    let app_secret = appstate.mxnzp_secret.clone();
    let url = format!(
        "https://www.mxnzp.com/api/verifycode/code?len={}&type=0&app_id={}&app_secret={}",
        4, app_id, app_secret
    );
    let mut verifycode = String::new();
    let mut verifycode_url = String::new();
    for _ in 0..5 {
        let json: serde_json::Value = reqwest::get(&url).await?.json().await?;
        match json["data"]["verifyCode"].as_str() {
            Some(x) => {
                verifycode = x.to_string().to_lowercase();
                verifycode_url = json["data"]["verifyCodeImgUrl"].to_string().trim_matches('"').to_string();
                break;
            }
            None => time::sleep(Duration::from_secs(1)).await,
        }
    }
    if verifycode.is_empty() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("获取验证码失败"));
        Ok(())
    } else {
        appstate
            .verifycode
            .insert(verifycode_url.clone(), (verifycode.clone(), Instant::now()));
        res.status_code(StatusCode::OK);
        res.render(verifycode_url);
        Ok(())
    }
}
