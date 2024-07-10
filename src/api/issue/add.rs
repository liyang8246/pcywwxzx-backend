use chrono::Local;
use salvo::prelude::*;
use crate::model::*;

#[handler]
pub async fn add_issue(req:&mut Request, depot: &mut Depot, res: &mut Response) {
    let mut appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let verify_issue: ResWithVerifyCode<Issue> = req.parse_json().await.unwrap();
    let verifycode_url = verify_issue.verifycode_url;
    if let Some(verifycode) = appstate.verifycode.get(&verifycode_url) {
        if verifycode.0 == verify_issue.verifycode {
            let mut issue = verify_issue.response;
            issue.reg_time = Some(Local::now().naive_local());
            issue.closed = Some(false);
            issue.closed_time = None;
            sqlx::query!(
                r#"INSERT INTO issue (uid, name, class, problem, reg_time, app_time, closed, closed_time) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
                issue.uid,
                issue.name,
                issue.class,
                issue.problem,
                issue.reg_time,
                issue.app_time,
                issue.closed,
                issue.closed_time,
            ).execute(&appstate.db_pool).await.unwrap();
            res.status_code(StatusCode::OK);
            res.render(Text::Plain("预约成功"));
        } else {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Text::Plain("验证码错误"));
        }
        appstate.verifycode.remove(&verifycode_url);
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("验证码过期"));
    }
}