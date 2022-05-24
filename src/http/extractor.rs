use crate::config::Config;
use crate::types::ghub::GitEvent;

use axum::body::Bytes;
use axum::extract::{FromRequest, RequestParts};
use axum::http::StatusCode;
use axum::Extension;

use hex::FromHex;
use hmac::digest::MacError;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use hyper::Body;

const X_HUB_SIG_256: &str = "x-hub-signature-256";
const SHA_PREFIX: &str = "sha256=";

// RA returns an error here about this not being dyn (?)
#[axum::async_trait]
impl FromRequest<Body> for GitEvent {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let ctx: Extension<Config> =
            Extension::from_request(req).await.expect("Application config is included");

        let body = req.take_body().unwrap();
        let sig = req.headers().get(X_HUB_SIG_256);

        if sig.is_none() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let sig = sig.unwrap().to_str().unwrap();
        let body = hyper::body::to_bytes(body).await.unwrap();

        match verify(&body, sig, ctx.github.hmac_key()) {
            Err(_) => Err(StatusCode::UNAUTHORIZED),
            Ok(()) => Ok(serde_json::from_slice(&body).unwrap()),
        }
    }
}

fn verify(payload_body: &Bytes, payload_hash: &str, secret: &str) -> anyhow::Result<(), MacError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload_body);

    let payload_hash = <Vec<u8>>::from_hex(&payload_hash[SHA_PREFIX.len()..]).unwrap();
    mac.verify_slice(&payload_hash)
}
