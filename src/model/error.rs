use salvo::prelude::*;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("reqwest: `{0}`")]
    Reqw(#[from] reqwest::Error),
    #[error("sqlx: `{0}`")]
    Sqlx(#[from] sqlx::Error),
    #[error("serde_json: `{0}`")]
    SerdeJson(#[from] serde_json::Error),
    #[error("parameter: `{0}`")]
    Parameter(&'static str),
    #[error("parse_json: `{0}`")]
    ParseJson(#[from] salvo::http::errors::ParseError),
}

pub type AppResult<T> = Result<T, AppError>;

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        error!("AppError: {:?}", self);
        res.render(Text::Plain("服务器内部错误"));
    }
}
