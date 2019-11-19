extern crate jsonwebtoken as jwt;

use crate::{
    config::db::Connection,
    config::jwt::{Claims, SECRET_KEY},
    schema::users::{self, dsl::*},
};
use actix_web::http::HeaderMap;
use diesel::prelude::*;
use jwt::errors::ErrorKind;
use jwt::{decode, Validation};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserForm {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn auth(headers: &HeaderMap, conn: &Connection) -> Result<User, String> {
        match Self::decode_token_from_headers(headers) {
            Ok(auth_email) => Ok(users
                .filter(email.eq(auth_email))
                .get_result::<User>(conn)
                .unwrap()),
            Err(err) => Err(err),
        }
    }

    pub fn auth_check(headers: &HeaderMap) -> bool {
        match Self::decode_token_from_headers(headers) {
            Ok(_token) => true,
            Err(_err) => false,
        }
    }

    pub fn decode_token_from_headers(headers: &HeaderMap) -> Result<String, String> {
        let token = match headers.get("authorization") {
            Some(v) => v.to_str().unwrap()[7..v.len()].to_string(),
            None => "".to_string(),
        };
        match decode::<Claims>(&token, SECRET_KEY.as_ref(), &Validation::default()) {
            Ok(c) => Ok(c.claims.sub),
            Err(err) => Err(match *err.kind() {
                ErrorKind::InvalidToken => "invalid token".to_string(),
                ErrorKind::InvalidSignature => "invalid signature".to_string(),
                ErrorKind::InvalidEcdsaKey => "invalid ECDSA key".to_string(),
                ErrorKind::InvalidRsaKey => "invalid RSA key".to_string(),
                ErrorKind::ExpiredSignature => "expired signature".to_string(),
                ErrorKind::InvalidIssuer => "invalid issuer".to_string(),
                ErrorKind::InvalidAudience => "invalid audience".to_string(),
                ErrorKind::InvalidSubject => "invalid subject".to_string(),
                ErrorKind::ImmatureSignature => "immature signature".to_string(),
                ErrorKind::InvalidAlgorithm => "algorithms don't match".to_string(),
                ErrorKind::Base64(ref _err) => "base64 error".to_string(),
                ErrorKind::Json(ref _err) => "json error".to_string(),
                ErrorKind::Utf8(ref _err) => "utf-8 error".to_string(),
                ErrorKind::Crypto(ref _err) => "crypto error".to_string(),
                _ => "unreachable".to_string(),
            }),
        }
    }

    pub fn find_user_by_email(email_address: &str, conn: &Connection) -> QueryResult<User> {
        users
            .filter(email.eq(email_address))
            .get_result::<User>(conn)
    }
}
