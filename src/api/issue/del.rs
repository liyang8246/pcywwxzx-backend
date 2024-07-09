use salvo::prelude::*;
use crate::model::*;

#[handler]
pub async fn del_issue(req:&mut Request, depot: &mut Depot, res: &mut Response) {
    let appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let passwd = req.query::<String>("passwd").unwrap();
    let id = req.query::<i64>("id").unwrap();
    if passwd != appstate.manager_passwd {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("密码错误"));
        return;
    }
    sqlx::query!(r#"DELETE FROM issue WHERE id = ?"#, id)
        .execute(&appstate.db_pool)
        .await
        .unwrap();
    res.status_code(StatusCode::OK);
    res.render(Text::Plain("维修单删除成功"));
}