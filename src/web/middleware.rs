use crate::{Error, Result};
use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn require_auth<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - middleware::require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: Real auth-token parsing & validation.
    auth_token.ok_or(Error::AuthTokenCookieNotFound)?;

    Ok(next.run(req).await)
}
