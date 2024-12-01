use crate::model::{AppResult, State};
use salvo::prelude::*;

#[handler]
pub async fn get_issue_num(depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let num = sqlx::query!("SELECT COUNT(*) FROM issue").fetch_one(&appstate.db_pool).await?.count;
    match num {
        Some(x) => res.render(Text::Plain(x.to_string())),
        None => res.render(Text::Plain("0".to_string())),
    }
    Ok(())
}

#[handler]
pub async fn get_date_num(depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let num = sqlx::query!("SELECT COUNT(DISTINCT app_time) FROM issue").fetch_one(&appstate.db_pool).await?.count;
    match num {
        Some(x) => res.render(Text::Plain(x.to_string())),
        None => res.render(Text::Plain("0".to_string())),
    }
    Ok(())
}