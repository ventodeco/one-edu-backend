pub mod commons {
    pub mod authentication {
        pub mod password_hash;
        pub mod auth_keys_service;
    }
    pub mod repositories {
        pub mod base;
        pub mod users {
            pub mod user_model;
            pub mod user_repository;
        }
        pub mod user_questions {
            pub mod user_question_model;
            pub mod user_question_repository;
        }
    }
}
pub mod app_state;
pub mod services {
    pub mod authentications {
        pub mod authentication_dto;
        pub mod authentication_service;
    }
    pub mod redis {
        pub mod redis_service;
        pub mod redis_helper;
    }
    pub mod user_questions {
        pub mod user_question_dto;
        pub mod user_question_service;
    }
}


use std::env;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::http::header;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use uuid::Uuid;
use crate::app_state::AppState;
use crate::commons::authentication::auth_keys_service::{init_auth_keys, AuthService};
use crate::commons::repositories::base::{DbRepo, Repository};
use crate::services::authentications::authentication_service::{login_user, register_user};
use crate::services::redis::redis_service::{RedisService, RedisSvc};
use crate::services::user_questions::user_question_service::{get_exam_summary, get_exam, start_exam};

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv().ok();

    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    // let allowed_domain = env::var("ALLOWED_DOMAIN").unwrap();

    let app_data = web::Data::new(AppState {
        repo: DbRepo::init().await,
        auth_service: AuthService,
        auth_keys: init_auth_keys().await,
        redis_service: RedisSvc::init().await
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    // .allowed_origin(&allowed_domain)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        header::CONTENT_TYPE,
                        header::AUTHORIZATION,
                        header::ACCEPT,
                    ])
                    .supports_credentials()
                    .max_age(3600)
            )
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/users")
                            .service(
                                web::resource("/register")
                                    .route(web::post().to(register_user::<DbRepo, AuthService, RedisSvc>))
                            )
                            .service(
                                web::resource("/login")
                                    .route(web::post().to(login_user::<DbRepo, AuthService, RedisSvc>))
                            )
                    )
                    .service(
                        web::scope("/exams")
                            .service(
                                web::resource("/{uuid}/summary")
                                    .route(web::get().to(|path: web::Path<Uuid>, request: actix_web::HttpRequest, data: web::Data<AppState<DbRepo, AuthService, RedisSvc>>| get_exam_summary(data, request, path.into_inner())))
                            )
                            .service(
                                web::resource("/{uuid}/start")
                                    .route(web::post().to(|path: web::Path<Uuid>, request: actix_web::HttpRequest, data: web::Data<AppState<DbRepo, AuthService, RedisSvc>>| start_exam(data, request, path.into_inner())))
                            )
                            .service(
                                web::resource("/{uuid}")
                                    .route(web::get().to(|path: web::Path<Uuid>, data: web::Data<AppState<DbRepo, AuthService, RedisSvc>>| get_exam(data, path.into_inner())))
                            )
                    )
            )
    })
    .bind((host, port)).expect("")
    // note: cannot use this for dev as client must also be on https,
    // enable at production
    // .bind_openssl((host, port), ssl_builder()).expect("SSL not working")
    .run()
    .await
}
