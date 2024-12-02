use std::collections::HashSet;

use crate::model::{AppResult, State};
use chrono::{Datelike, NaiveDateTime};
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
    let datetimes:Vec<NaiveDateTime> = sqlx::query!("SELECT app_time FROM issue").fetch_all(&appstate.db_pool).await?.iter().map(|x|x.app_time).collect();
    let mut weeks = HashSet::new();
    for datetime in datetimes {
        let weekday = datetime.weekday();
        let monday = datetime - chrono::Duration::days(weekday.num_days_from_monday() as i64);
        weeks.insert(monday.date());
    }
    let days = weeks.len() * 5;
    res.render(Text::Plain(days.to_string()));
    Ok(())
}