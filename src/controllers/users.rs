use actix_web::{
    error, 
    web, 
    Error, 
    HttpResponse,
    HttpRequest, 
    Result,
};
use sea_orm::*;
use serde_json::json;
use bcrypt::{hash, DEFAULT_COST};
use crate::configures::options::{AppState, Params};
use crate::models::{user, user::Entity as User};


pub async fn list_users(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let posts_per_page = params.posts_per_page.unwrap_or(10);
    let (users, num_pages) = find_users_in_page(conn, page, posts_per_page)
        .await
        .expect("Cannot find users in page");

    Ok(HttpResponse::Ok().json(json!({
        "users": &users,
        "page": &page,
        "posts_per_page": &posts_per_page,
        "num_pages": &num_pages,
    })))

}

async fn find_users_in_page(
    db: &DbConn,
    page: u64,
    posts_per_page: u64,
) -> Result<(Vec<user::Model>, u64), DbErr> {
        // Setup paginator
    let paginator = User::find()
        .order_by_asc(user::Column::Id)
        .paginate(db, posts_per_page);
    let num_pages = paginator.num_pages().await?;

    paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
}

pub async fn create_user(
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

pub async fn edit_user(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    let users: Option<user::Model> = find_user_by_id(conn, id)
        .await
        .expect("could not find user");

    Ok(HttpResponse::Ok().json(json!({
        "users": &users,
    })))
}


pub async fn update_user(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    post_form: web::Form<user::Model>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = post_form.into_inner();
    let id = id.into_inner();

    let user_request: Option<user::Model> = find_user_by_id(conn, id)
        .await
        .expect("could not find user");

    let user_id = match user_request {
        Some(user) => {
            user.id
        },
        None => {
            return Ok(HttpResponse::NotFound().finish());
        }
    };
    
    let hashed_password = hash(form.password.to_owned(), DEFAULT_COST)
        .map_err(|_| error::ErrorInternalServerError("Failed to hash password"))?;

    let _ = user::ActiveModel {
        id: Set(user_id),
        email: Set(form.email.to_owned()),
        name: Set(form.name.to_owned()),
        password: Set(hashed_password),
        role: Set(form.role.to_owned()),
        }
        .update(conn)
        .await;

    Ok(HttpResponse::Ok().json(json!({
        "message": "ok",
    })))
}

pub async fn delete_user(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
  let user_request: Option<user::Model> = find_user_by_id(conn, id)
        .await
        .expect("could not find user");

    let user_data = match user_request {
        Some(user) => {
            user
        },
        None => {
            return Ok(HttpResponse::NotFound().finish());        }
    };

    let _ = user_data.delete(conn).await;

    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}

async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
}
