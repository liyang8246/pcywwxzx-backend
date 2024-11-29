use crate::model::*;
use salvo::prelude::*;

#[handler]
pub async fn view_issue(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let passwd = req.query::<String>("passwd").ok_or(AppError::Parameter("passwd"))?;
    if passwd != appstate.manager_passwd {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("密码错误"));
        return Ok(());
    }
    let mut issues = sqlx::query!("SELECT * FROM issue")
        .fetch_all(&appstate.db_pool)
        .await?
        .into_iter()
        .map(|issue| Issue {
            id: Some(issue.id as usize),
            uid: Some(issue.uid),
            name: Some(issue.name),
            class: Some(issue.class),
            phone: Some(issue.phone),
            problem: Some(issue.problem),
            reg_time: Some(issue.reg_time),
            app_time: Some(issue.app_time),
            closed: Some(issue.closed),
            closed_time: issue.closed_time,
        })
        .collect::<Vec<Issue>>();
    drop(appstate);
    issues.sort();
    let json = serde_json::to_string(&issues)?;
    res.status_code(StatusCode::OK);
    res.render(Text::Json(json));
    Ok(())
}
