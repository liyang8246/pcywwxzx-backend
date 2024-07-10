use salvo::prelude::*;
use crate::model::*;

#[handler]
pub async fn view_issue(req:&mut Request, depot: &mut Depot, res: &mut Response) {
    let appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let passwd = req.query::<String>("passwd").unwrap();
    if passwd != appstate.manager_passwd {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Text::Plain("密码错误"));
    } else {
        let mut issues = sqlx::query!("SELECT * FROM issue")
            .fetch_all(&appstate.db_pool)
            .await
            .unwrap().into_iter()
            .map(|issue| Issue {
                id: Some(issue.id as usize),
                uid: Some(issue.uid.to_string()),
                name: Some(issue.name),
                class: Some(issue.class),
                problem: Some(issue.problem),
                reg_time: Some(issue.reg_time),
                app_time: Some(issue.app_time),
                closed: Some(issue.closed),
                closed_time: issue.closed_time,
            }).collect::<Vec<Issue>>();
        drop(appstate);
        issues.sort();
        let json = serde_json::to_string(&issues).unwrap();
        res.status_code(StatusCode::OK);
        res.render(Text::Json(json));
    }
}