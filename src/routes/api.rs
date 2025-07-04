use actix_web::{web, middleware::from_fn};
use crate::middlewares::jwt::jwt_middleware;
use crate::controllers::auth::{
    user_registration_post, 
    user_auth_post, 
    tests,
};
use crate::controllers::users::{
    list_users,
    create_user, 
    edit_user,
    update_user, 
    delete_user, 
};
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/signup")
                    .route(web::post().to(user_registration_post)),
            )
            .service(
                web::resource("/signin")
                    .route(web::post().to(user_auth_post)),
            )
            .service(
                web::resource("/test")
                    .route(web::get().to(tests))
                    .wrap(from_fn(jwt_middleware)),
            ),
    );
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("")
                    .route(web::get().to(list_users))
                    .route(web::post().to(create_user))
                    .wrap(from_fn(jwt_middleware)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(edit_user))
                    .route(web::patch().to(update_user))
                    .route(web::delete().to(delete_user))
                    .wrap(from_fn(jwt_middleware)),
            ),
    );
}
