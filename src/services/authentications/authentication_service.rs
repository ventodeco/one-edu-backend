use actix_web::{
    web::{Data, Json}, 
    HttpResponse,
    http::header::ContentType
};
use actix_web::cookie::Cookie;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Validation};
use log::error;
use uuid::Uuid;
use crate::{
    app_state::AppState, 
    commons::{
        authentication::auth_keys_service::{
            Authenticator
        }
    }, 
};
use crate::commons::authentication::auth_keys_service::{Claims, REFRESH_TOKEN_LABEL, STANDARD_ACCESS_TOKEN_EXPIRATION, STANDARD_REFRESH_TOKEN_EXPIRATION};
use crate::commons::repositories::base::Repository;
use crate::commons::repositories::users::user_model::NewUser;
use crate::commons::repositories::users::user_repository::UserRepository;
use crate::services::redis::redis_helper::RedisHelper;
use crate::services::redis::redis_service::RedisService;
use super::authentication_dto::{GenericError, GenericResponse, LoginRequest, RegisterRequest, RegisterResponse};

pub async fn register_user<
    T: UserRepository + Repository,
    U: Authenticator,
    V: RedisService
    >(app_data: Data<AppState<T, U, V>>, register_request: Json<RegisterRequest>) -> Result<GenericResponse, GenericResponse>
{

    let result = app_data.repo.register_user(
        NewUser {
            uuid: Uuid::now_v7(),
            username: register_request.user_name.clone(),
            full_name: register_request.full_name.clone(),
            email: register_request.email.clone(),
            phone_number: register_request.phone_number.clone(),
            // university: register_request.university.clone(),
            // major: register_request.major.clone(),
            role: "USER".to_string(),
            password: register_request.password.clone(),
        }
    ).await;

    match result {
        Ok(entity) => Ok(
            GenericResponse {
                success: true,
                data: Option::from(RegisterResponse {
                    id: entity.id.to_string()
                }),
                error: None
            }
        ),
        Err(e) => Ok(
            GenericResponse {
                success: false,
                data: None,
                error: Option::from(GenericError {
                    code: 500,
                    entity: "testing".to_string(),
                    message: e.to_string()
                }),
            }
        )
    }
}

pub async fn login_user<
    T: UserRepository + Repository,
    U: Authenticator,
    V: RedisHelper + RedisService
    >(app_data: Data<AppState<T, U, V>>, login_request: Json<LoginRequest>)
      -> HttpResponse {

    let redis_key = "test".to_string();
    let redis_value = "uhuy".to_string();
    let val = app_data.redis_service.set(redis_key.clone(), redis_value).await;
    let val_set = app_data.redis_service.get(redis_key).await;

    println!("val {:?}", val);
    println!("val_set {:?}", val_set);

    let auth_result = app_data.repo.login(login_request.email.clone(), login_request.password.clone()).await;

    match auth_result {
        Ok(_) => {
            let user_name = "".to_string();
            let (refresh_cookie, access_token) = get_refresh_and_access_token_response(app_data, user_name.as_str());
            HttpResponse::Ok()
                .cookie(refresh_cookie)
                .body(access_token)
        }
        Err(_) => {
            error!("Authentication failed. Server error");
            HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body("Authentication failed. Server error occurred while trying to authenticate")
        }
    }
}

fn get_refresh_and_access_token_response<'a, T: Repository, U: Authenticator, V: RedisService>(
    app_data: Data<AppState<T, U, V>>, user_name: &'a str
) -> (Cookie<'a>, String) {
    let access_token = get_token(user_name.to_string(), &app_data.auth_keys.encoding_key, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));
    let refresh_token = get_token(user_name.to_string(), &app_data.auth_keys.encoding_key, None);
    let refresh_cookie = Cookie::build(REFRESH_TOKEN_LABEL, refresh_token.to_owned())
        .path("/")
        .max_age(actix_web::cookie::time::Duration::new(STANDARD_REFRESH_TOKEN_EXPIRATION, 0))
        .http_only(true)
        .secure(false)
        //.same_site(SameSite::Lax)
        .finish();

    (refresh_cookie, access_token)
}

pub fn get_token(user_name: String, encoding_key: &EncodingKey, exp_duration_seconds: Option<i64>) -> String {
    let duration = if let None = exp_duration_seconds {
        STANDARD_REFRESH_TOKEN_EXPIRATION
    } else {
        exp_duration_seconds.unwrap()
    };
    let claims = Claims { sub: user_name, exp: (Utc::now() + Duration::seconds(duration)).timestamp() as usize };
    let token = encode(&jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA), &claims, encoding_key).unwrap();

    token
}

pub fn decode_token(token: &str, decoding_key: &DecodingKey) -> Claims {
    let validation = Validation::new(Algorithm::EdDSA);
    let token_data = decode::<Claims>(token, decoding_key, &validation).unwrap();

    token_data.claims
}
