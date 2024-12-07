use std::collections::HashSet;

use crate::model::{AppResult, State};
use chrono::{Datelike, NaiveDateTime};
use salvo::prelude::*;

#[handler]
pub async fn get_issue_num(depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let appstate = depot.obtain::<State>().expect("get appstate fail").read().await;
    let num = sqlx::query!("SELECT COUNT(*) FROM issue WHERE closed = true")
        .fetch_one(&appstate.db_pool)
        .await?
        .count;
    match num {
        Some(x) => res.render(Text::Plain(x.to_string())),
        None => res.render(Text::Plain("0".to_string())),
    }
    Ok(())
}

#[handler]
pub async fn get_date_num(depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let appstate = depot.obtain::<State>().expect("get appstate fail").read().await;
    let datetimes: Vec<NaiveDateTime> = sqlx::query!("SELECT app_time FROM issue")
        .fetch_all(&appstate.db_pool)
        .await?
        .iter()
        .map(|x| x.app_time)
        .collect();
    if datetimes.is_empty() {
        res.render(Text::Plain("0".to_string()));
        return Ok(());
    }
    let mut weeks = HashSet::new();
    for datetime in &datetimes {
        let weekday = datetime.weekday();
        let monday = *datetime - chrono::Duration::days(weekday.num_days_from_monday() as i64);
        weeks.insert(monday.date());
    }
    let last_week_day = datetimes.iter().max().unwrap().weekday().num_days_from_monday() as usize;
    let days = (weeks.len() - 1) * 5 + last_week_day;
    res.render(Text::Plain(days.to_string()));
    Ok(())
}
