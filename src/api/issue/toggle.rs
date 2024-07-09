use chrono::Local;
use salvo::prelude::*;
use crate::model::*;

#[handler]
pub async fn toggle_issue(req:&mut Request, depot: &mut Depot, res: &mut Response) {
    let appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let passwd = req.query::<String>("passwd").unwrap();
    let id = req.query::<i64>("id").unwrap();
    if passwd != appstate.manager_passwd {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("密码错误"));
        return;
    }
    let issue = sqlx::query!("SELECT * FROM issue WHERE id = ?", id)
        .fetch_one(&appstate.db_pool)
        .await
        .unwrap();
    let closed = !issue.closed;
    let closed_time = if closed { Some(Local::now().naive_local()) } else { None };
    sqlx::query!(r#"UPDATE issue SET closed = ?, closed_time = ? WHERE id = ?"#,
        closed,
        closed_time,
        id,
    ).execute(&appstate.db_pool).await.unwrap();
    res.status_code(StatusCode::OK);
    res.render(Text::Plain("维修状态切换成功"))
}