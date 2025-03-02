use actix_web::{
    web::{Data, Json}
};
use actix_web::cookie::Cookie;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Validation};
use log::{error, info};
use uuid::Uuid;
use regex::Regex;
use sqlx::Error;
use crate::{
    app_state::AppState, 
    commons::{
        authentication::auth_keys_service::{
            Authenticator
        }
    }, 
};
use crate::commons::authentication::auth_keys_service::{Claims, REFRESH_TOKEN_LABEL, STANDARD_ACCESS_TOKEN_EXPIRATION, STANDARD_REFRESH_TOKEN_EXPIRATION};
use crate::commons::instrumentation::statsd_config::StatsdService;
use crate::commons::repositories::base::Repository;
use crate::commons::repositories::users::user_model::NewUser;
use crate::commons::repositories::users::user_repository::UserRepository;
use crate::services::redis::redis_helper::RedisHelper;
use crate::services::redis::redis_service::RedisService;
use super::authentication_dto::{GenericError, GenericResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};

pub async fn register_user<
    T: UserRepository + Repository,
    U: Authenticator,
    V: RedisService,
    W: StatsdService
    >(app_data: Data<AppState<T, U, V, W>>, register_request: Json<RegisterRequest>) -> Result<GenericResponse<RegisterResponse>, GenericResponse<RegisterResponse>>
{

    let user_exist = app_data.repo.check_user_existence(register_request.user_name.clone(), register_request.email.clone(), register_request.phone_number.clone(), register_request.partner.clone()).await;

    match user_exist {
        Ok(exist) if exist => {
            let email_exist = app_data.repo.check_user_by_email(register_request.email.clone(), register_request.partner.clone()).await;

            match email_exist {
                Ok(exists) if exists => {
                    return Ok(
                        GenericResponse {
                            success: false,
                            data: None,
                            error: Option::from(GenericError {
                                code: 1007,
                                entity: "ONE_EDU_BACKEND".to_string(),
                                message: "EMAIL_ALREADY_REGISTERED".to_string()
                            }),
                        }
                    )
                },
                Ok(_) => {},
                Err(_) => {}
            }

            let phone_exist = app_data.repo.check_user_by_phone_number(register_request.phone_number.clone(), register_request.partner.clone()).await;

            match phone_exist {
                Ok(exists) if exists => {
                    return Ok(
                        GenericResponse {
                            success: false,
                            data: None,
                            error: Option::from(GenericError {
                                code: 1005,
                                entity: "ONE_EDU_BACKEND".to_string(),
                                message: "PHONE_NUMBER_ALREADY_REGISTERED".to_string()
                            }),
                        }
                    )
                },
                Ok(_) => {},
                Err(_) => {}
            }

            let username_exist = app_data.repo.check_user_by_username(register_request.user_name.clone(), register_request.partner.clone()).await;

            match username_exist {
                Ok(exists) if exists => {
                    return Ok(
                        GenericResponse {
                            success: false,
                            data: None,
                            error: Option::from(GenericError {
                                code: 1006,
                                entity: "ONE_EDU_BACKEND".to_string(),
                                message: "USER_NAME_ALREADY_REGISTERED".to_string()
                            }),
                        }
                    )
                },
                Ok(_) => {},
                Err(_) => {}
            }

            return Ok(
                GenericResponse {
                    success: false,
                    data: None,
                    error: Option::from(GenericError {
                        code: 500,
                        entity: "ONE_EDU_BACKEND".to_string(),
                        message: "SERVER_ERROR".to_string()
                    }),
                }
            )
        },
        Ok(_) => {},
        Err(_) => {}
    }

    let result = app_data.repo.register_user(
        NewUser {
            uuid: Uuid::now_v7(),
            username: register_request.user_name.clone(),
            full_name: register_request.full_name.clone(),
            email: register_request.email.clone(),
            phone_number: register_request.phone_number.clone(),
            partner: register_request.partner.clone(),
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
                    entity: "ONE_EDU_BACKEND".to_string(),
                    message: e.to_string()
                }),
            }
        )
    }
}

pub async fn login_user<
    T: UserRepository + Repository,
    U: Authenticator,
    V: RedisHelper + RedisService,
    W: StatsdService
    >(app_data: Data<AppState<T, U, V, W>>, login_request: Json<LoginRequest>)
      -> Result<GenericResponse<LoginResponse>, GenericResponse<LoginResponse>> {

    let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    let phone_regex = Regex::new(r"^081\d+$").unwrap();

    let auth_result = if email_regex.is_match(&login_request.user_identifier) {
        info!("Logging in via email {} and partner {}", login_request.user_identifier, login_request.partner);
        app_data.repo.login_via_email(login_request.user_identifier.clone(), login_request.password.clone(), login_request.partner.clone()).await
    } else if phone_regex.is_match(&login_request.user_identifier) {
        info!("Logging in via phone number {} and partner {}", login_request.user_identifier, login_request.partner);
        app_data.repo.login_via_phone_number(login_request.user_identifier.clone(), login_request.password.clone(), login_request.partner.clone()).await
    } else {
        info!("Logging in via username {} and partner {}", login_request.user_identifier, login_request.partner);
        app_data.repo.login_via_user_name(login_request.user_identifier.clone(), login_request.password.clone(), login_request.partner.clone()).await
    };

    match auth_result {
        Ok(_) => {
            let user = auth_result.unwrap();
            let user_name = user.user_name;
            let user_id = user.id;
            let (_, access_token) = get_refresh_and_access_token_response(app_data, user_name.as_str(), user_id);
            Ok(
                    GenericResponse {
                        success: true,
                        data: Option::from(LoginResponse {
                            access_token
                        }),
                        error: None
                    }
            )
        }
        Err(_) => {
            error!("Authentication failed. Server error");
            Ok(
                GenericResponse {
                    success: false,
                    data: None,
                    error: Option::from(GenericError {
                        code: 500,
                        entity: "testing".to_string(),
                        message: "Authentication failed. Server error occurred while trying to authenticate".to_string()
                    }),
                }
            )
        }
    }
}

fn get_refresh_and_access_token_response<'a, T: Repository, U: Authenticator, V: RedisService, W: StatsdService>(
    app_data: Data<AppState<T, U, V, W>>, user_name: &'a str, user_id: i64
) -> (Cookie<'a>, String) {
    let access_token = get_token(user_name.to_string(), &app_data.auth_keys.encoding_key, Some(STANDARD_ACCESS_TOKEN_EXPIRATION), user_id);
    let refresh_token = get_token(user_name.to_string(), &app_data.auth_keys.encoding_key, None, user_id);
    let refresh_cookie = Cookie::build(REFRESH_TOKEN_LABEL, refresh_token.to_owned())
        .path("/")
        .max_age(actix_web::cookie::time::Duration::new(STANDARD_REFRESH_TOKEN_EXPIRATION, 0))
        .http_only(true)
        .secure(false)
        //.same_site(SameSite::Lax)
        .finish();

    (refresh_cookie, access_token)
}

pub fn get_token(user_name: String, encoding_key: &EncodingKey, exp_duration_seconds: Option<i64>, user_id: i64) -> String {
    let duration = if let None = exp_duration_seconds {
        STANDARD_REFRESH_TOKEN_EXPIRATION
    } else {
        exp_duration_seconds.unwrap()
    };
    let claims = Claims { sub: user_name, exp: (Utc::now() + Duration::seconds(duration)).timestamp() as usize, user_id };
    let token = encode(&jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA), &claims, encoding_key).unwrap();

    token
}

pub fn decode_token(token: &str, decoding_key: &DecodingKey) -> Claims {
    let validation = Validation::new(Algorithm::EdDSA);
    let token_data = decode::<Claims>(token, decoding_key, &validation).unwrap();

    token_data.claims
}
