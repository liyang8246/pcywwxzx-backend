use crate::model::*;
use salvo::prelude::*;

#[handler]
pub async fn del_issue(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let passwd = req.query::<String>("passwd").ok_or(AppError::Parameter("passwd"))?;
    let id = req.query::<i32>("id").ok_or(AppError::Parameter("id"))?;
    if passwd != appstate.manager_passwd {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("密码错误"));
        return Ok(());
    }
    sqlx::query!(r#"DELETE FROM issue WHERE id = $1"#, id)
        .execute(&appstate.db_pool)
        .await?;
    res.status_code(StatusCode::OK);
    res.render(Text::Plain("维修单删除成功"));
    Ok(())
}
