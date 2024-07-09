use jsonwebtoken::EncodingKey;
use salvo::{http::cookie::CookieBuilder, prelude::*};
use time::{Duration, OffsetDateTime};

use crate::{model::jwt::JwtClaims, SECRET_KEY, TOKEN_EXP_TIME};

#[handler]
pub async fn set_jwt_cookie(req: &mut Request, res: &mut Response) {
    let username = match req.query::<String>("name") {
        Some(x) => x,
        None => {
            res.status_code(StatusCode::BAD_REQUEST);
            return;
        }
    };
    if username != "liyang" {
        res.status_code(StatusCode::UNAUTHORIZED);
        return;
    }
    let claim = JwtClaims {
        username,
        exp: (OffsetDateTime::now_utc() + Duration::seconds(TOKEN_EXP_TIME)).unix_timestamp(),
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .unwrap();
    res.add_cookie(
        CookieBuilder::new("jwt_token", token)
            .path("/")
            .max_age(Duration::seconds(TOKEN_EXP_TIME))
            .build(),
    );
}
