use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    HttpResponse,
    Error,
};

use crate::configures::options::{get_jwt_secret_key, Claims};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn jwt_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    // Извлечение токена из заголовка Authorization
    let token = req.headers().get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let token = token.strip_prefix("Bearer ").unwrap_or("");
    println!("{}", token);
    // Настройка валидации JWT
    let validation = Validation::default();
    let secret_key = get_jwt_secret_key();
    let decoding_key = DecodingKey::from_secret(&secret_key);
    println!("{:?}", secret_key);

    // Декодирование токена
    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(_) => {
            // Если токен валиден, продолжаем выполнение следующего middleware или обработчика
            let response = next.call(req).await?.map_into_boxed_body();
            Ok(response)
        }
        Err(_) => {
                        // Если токен невалиден, возвращаем ответ с ошибкой
            let response = HttpResponse::Unauthorized()
                .content_type("application/json")
                .body(r#"{"error": "Unauthorized: Invalid token"}"#);

            // Создаем новый ServiceResponse с ошибкой
            let service_response = ServiceResponse::new(req.request().clone(), response);
            Ok(service_response)
                     // Если токен невалиден, возвращаем ответ с ошибкой
        }
    }
}
