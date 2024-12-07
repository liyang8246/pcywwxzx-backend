use crate::model::*;
use chrono::Local;
use salvo::prelude::*;
use tracing::info;

#[handler]
pub async fn add_issue(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let mut appstate = depot.obtain::<State>().expect("get db_pool fail").write().await;
    let verify_issue: ResWithVerifyCode<Issue> = req.parse_json().await?;
    let verifycode_url = verify_issue.verifycode_url;
    let verifycode = appstate.verifycode.get(&verifycode_url);
    if verifycode.is_none() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("验证码过期"));
        return Ok(());
    }
    if verifycode.unwrap().0 != verify_issue.verifycode {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("验证码错误"));
        return Ok(());
    }
    let mut issue = verify_issue.response;
    issue.reg_time = Some(Local::now().naive_local());
    issue.closed = Some(false);
    issue.closed_time = None;
    sqlx::query!(
        r#"INSERT INTO issue 
                (uid, name, class, problem, phone, reg_time, app_time, closed, closed_time)
            VALUES
                ($1,$2,$3,$4,$5,$6,$7,$8,$9)"#,
        issue.uid,
        issue.name,
        issue.class,
        issue.problem,
        issue.phone,
        issue.reg_time,
        issue.app_time,
        issue.closed,
        issue.closed_time,
    )
    .execute(&appstate.db_pool)
    .await?;
    res.status_code(StatusCode::OK);
    res.render(Text::Plain("预约成功"));
    info!("add issue {}", issue);
    appstate.verifycode.remove(&verifycode_url);
    return Ok(());
}
