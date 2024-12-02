use crate::model::{AppResult, State};
use salvo::prelude::*;

#[handler]
pub async fn get_version(depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let appstate = depot.obtain::<State>().expect("get db_pool fail").lock().await;
    let version = &appstate.version;
    res.render(Text::Plain(version));
    Ok(())
}