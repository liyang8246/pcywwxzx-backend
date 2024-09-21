use crate::model::*;
use chrono::Local;
use salvo::prelude::*;

#[handler]
pub async fn toggle_issue(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let passwd = req.query::<String>("passwd").ok_or(AppError::Parameter("passwd"))?;
    let id = req.query::<i64>("id").ok_or(AppError::Parameter("id"))?;
    if passwd != appstate.manager_passwd {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("密码错误"));
        return Ok(());
    }
    let issue = sqlx::query!("SELECT * FROM issue WHERE id = ?", id)
        .fetch_one(&appstate.db_pool)
        .await?;
    let closed = !issue.closed;
    let closed_time = if closed { Some(Local::now().naive_local()) } else { None };
    sqlx::query!(
        r#"UPDATE issue SET closed = ?, closed_time = ? WHERE id = ?"#,
        closed,
        closed_time,
        id,
    )
    .execute(&appstate.db_pool)
    .await?;
    res.status_code(StatusCode::OK);
    res.render(Text::Plain("维修状态切换成功"));
    Ok(())
}
