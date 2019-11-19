extern crate jsonwebtoken as jwt;

use crate::{
    app::models::response::ResponseBody,
    app::models::user::{User, UserForm},
    config::db::Pool,
    config::jwt::{Claims, EXP, SECRET_KEY},
    error::ServiceError,
    schema::users::dsl::*,
};
use actix_web::{http::StatusCode, web, Error, HttpRequest, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use futures::future::{ok, Future};
use jwt::{encode, Header};

// POST api/auth/register
pub fn register(
    user_request: web::Json<UserForm>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let conn = &pool.get().unwrap();
    if User::find_user_by_email(&user_request.email, conn).is_err() {
        let hashed_pwd = hash(&user_request.password, DEFAULT_COST).unwrap();
        let user = UserForm {
            password: hashed_pwd,
            ..user_request.0
        };
        let _result = diesel::insert_into(users).values(&user).execute(conn);
        ok(HttpResponse::Ok().json(ResponseBody::new("SUCCESS", &user.email)))
    } else {
        ok(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{} this email address in use.", &user_request.email),
        )
        .response())
    }
}

// POST api/auth/login
pub fn login(
    user_request: web::Json<UserForm>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let conn = &pool.get().unwrap();
    let user = users
        .filter(email.eq(&user_request.email))
        .get_result::<User>(conn)
        .unwrap();
    if !user.password.is_empty() && verify(&user_request.password, &user.password).unwrap() {
        let my_claims = Claims {
            sub: user.email.to_owned(),
            exp: EXP,
        };
        match encode(&Header::default(), &my_claims, SECRET_KEY.as_ref()) {
            Ok(t) => ok(HttpResponse::Ok().json(ResponseBody::new("SUCCESS", t))),
            Err(_) => ok(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Encode Err".to_string(),
            )
            .response()),
        }
    } else {
        ok(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Email or Password incorrect!".to_string(),
        )
        .response())
    }
}

#[derive(Serialize, Deserialize)]
pub struct MeData {
    pub id: i32,
    pub email: String,
}

// GET api/auth/me
pub fn me(
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let headers = req.headers();
    if User::auth_check(headers) {
        match User::auth(headers, &pool.get().unwrap()) {
            Ok(auth_user) => {
                let data = MeData {
                    id: auth_user.id,
                    email: auth_user.email,
                };
                ok(HttpResponse::Ok().json(ResponseBody::new("SUCCESS", data)))
            }
            Err(err) => ok(HttpResponse::BadRequest().json(ResponseBody::new("ERROR", err))),
        }
    } else {
        ok(HttpResponse::BadRequest().json(ResponseBody::new("ERROR", "")))
    }
}
