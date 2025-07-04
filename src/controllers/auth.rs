use actix_web::{
    error, 
    web, 
    Error, 
    HttpResponse, 
    Result,
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde_json::json;
use chrono::{Utc, Duration};
use crate::configures::options::{AppState, Claims, get_jwt_secret_key};
use crate::models::user;

#[derive(Serialize, Deserialize)]
pub struct LoginValid {
    pub email: String,
    pub password: String,
}

pub async fn tests(
) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Created()
        .json("ok"))
}
pub async fn user_registration_post(
    data: web::Data<AppState>,
    post_form: web::Json<user::Model>,
) -> Result<HttpResponse, Error> {

    let conn = &data.conn;

    let form = post_form.into_inner();

    let hashed_password = hash(form.password.to_owned(), DEFAULT_COST)
        .map_err(|_| error::ErrorInternalServerError("Failed to hash password"))?;

    user::ActiveModel {
            name: Set(form.name.to_owned()),
            password: Set(hashed_password),
            email: Set(form.email.to_owned()),
            role: Set(form.role.to_owned()),
            ..Default::default()
        }
        .save(conn)
        .await
        .expect("could not insert user");
    Ok(HttpResponse::Created()
        .json("ok"))
}

pub async fn user_auth_post(
    data: web::Data<AppState>, 
    post_form: web::Json<LoginValid>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    let user_one: Option<user::Model> = find_user_by_email(conn, &form.email.to_owned())
        .await
        .expect("could not find user");

    match user_one {
        Some(user_request) => {
            match verify(&form.password.to_owned(), &user_request.password) {
                Ok(is_valid) if is_valid => {
                    {
                    // Generate JWT
                    let claims = Claims {
                        sub: user_request.email.clone(),
                        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize, // 1 hour expiration
                    };
                    let secret_key = get_jwt_secret_key();
                    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(&secret_key))
                        .map_err(|_| error::ErrorInternalServerError("Failed to create token"))?;

                    Ok(HttpResponse::Ok().json(json!({
                        "token": token,
                        "user": user_request
                    })))
                }
                }
                Ok(_) => {
                    Ok(HttpResponse::Unauthorized().body("Неверные учетные данные")) 
                }
                Err(_) => {
                    Ok(HttpResponse::InternalServerError().body("Ошибка при проверке пароля")) 
                }
            }
        }
        None => {
            Ok(HttpResponse::NotFound().body("Пользователь не найден")) 
        }
    }
}

async fn find_user_by_email(
    db: &DbConn, 
    email_auth: &str,
) -> Result<Option<user::Model>, DbErr> {
    let user_request = user::Entity::find()
        .filter(user::Column::Email.eq(email_auth)) 
        .one(db) 
        .await?;

    Ok(user_request) 
}
