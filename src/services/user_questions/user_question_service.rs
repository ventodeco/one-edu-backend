use std::ops::Add;
use actix_web::web::Data;
use chrono::Utc;
use log::{debug, info};
use uuid::Uuid;
use crate::app_state::AppState;
use crate::commons::authentication::auth_keys_service::Authenticator;
use crate::commons::repositories::base::Repository;
use crate::commons::repositories::user_questions::user_question_model::{QuestionDetails, UserQuestion};
use crate::commons::repositories::user_questions::user_question_repository::UserQuestionRepository;
use crate::services::authentications::authentication_dto::{GenericError, GenericResponse};
use crate::services::redis::redis_helper::RedisHelper;
use crate::services::redis::redis_service::RedisService;
use crate::services::user_questions::user_question_dto::{GetExamSummaryData};

pub async fn get_exam_summary<
    T: UserQuestionRepository + Repository,
    U: Authenticator,
    V: RedisService
>(
    app_data: Data<AppState<T, U, V>>, req: actix_web::HttpRequest, uuid: Uuid) -> Result<GenericResponse<GetExamSummaryData>, GenericResponse<()>> {

    log_mdc::insert("request_id", uuid.to_string());
    info!("GetExamSummary | Start getting exam summary");

    let headers = req.headers().iter().map(|(k, v)| (k.as_str(), v.to_str().unwrap())).collect();

    let auth_user_id = match check_authentication(&app_data, headers).await {
        Ok(user_id) => user_id,
        Err(err) => return Err(err),
    };

    let user_question: UserQuestion = app_data.repo.get_user_question_by_uuid(uuid).await.unwrap();

    if user_question.user_id != auth_user_id {
        return Err(GenericResponse {
            success: false,
            data: None,
            error: Some(GenericError {
                code: 401,
                entity: "ONE_EDU_APP".to_string(),
                message: "Unauthorized".to_string()
            })
        });
    }

    Ok(GenericResponse {
        success: true,
        data: Some(GetExamSummaryData {
            uuid,
            title: user_question.question_details.title,
            description: user_question.question_details.description.unwrap(),
            total_questions: user_question.question_details.total_questions,
            time_limit: user_question.question_details.time_limit,
        }),
        error: None
    })
}

pub async fn start_exam<
    T: UserQuestionRepository + Repository,
    U: Authenticator,
    V: RedisHelper + RedisService
>(
    app_data: Data<AppState<T, U, V>>, req: actix_web::HttpRequest, uuid: Uuid) -> Result<GenericResponse<QuestionDetails>, GenericResponse<()>> {

    log_mdc::insert("request_id", uuid.to_string());
    info!("StartExam | Start the exam");

    let headers = req.headers().iter().map(|(k, v)| (k.as_str(), v.to_str().unwrap())).collect();

    let auth_user_id = match check_authentication(&app_data, headers).await {
        Ok(user_id) => user_id,
        Err(err) => return Err(err),
    };

    let user_question = match app_data.repo.get_user_question_by_uuid(uuid).await {
        Ok(user_question) => user_question,
        Err(_) => {
            return Err(GenericResponse {
                success: false,
                data: None,
                error: Some(GenericError {
                    code: 404,
                    entity: "ONE_EDU_APP".to_string(),
                    message: "Not found".to_string()
                })
            });
        }
    };

    if user_question.user_id != auth_user_id {
        return Err(GenericResponse {
            success: false,
            data: None,
            error: Some(GenericError {
                code: 401,
                entity: "ONE_EDU_APP".to_string(),
                message: "Unauthorized".to_string()
            })
        });
    }

    if user_question.status == "IN_PROGRESS" {
        return Err(GenericResponse {
            success: false,
            data: None,
            error: Some(GenericError {
                code: 1001,
                entity: "ONE_EDU_APP".to_string(),
                message: "EXAM_ALREADY_STARTED".to_string()
            })
        });
    }

    let started_time = Utc::now();
    let ended_time = started_time.add(chrono::Duration::minutes(user_question.time_limit.unwrap() as i64));
    let mut question_details = user_question.question_details.clone();

    question_details.answered_questions = Some(0);
    question_details.answered_question_list = Some(Vec::new());
    question_details.started_at = Some(started_time);
    question_details.ended_at = Some(ended_time);

    info!("StartExam | Update data for start exam {:?}", question_details);

    app_data.repo.update_data_for_start_exam(user_question.uuid, started_time, ended_time, question_details.clone()).await.unwrap();

    app_data.redis_service.set(construct_exam_key(uuid), serde_json::to_string(&question_details).unwrap()).await.unwrap();

    Ok(GenericResponse {
        success: true,
        data: Some(question_details),
        error: None
    })
}

fn construct_exam_key(uuid: Uuid) -> String {
    format!("exam:{}", uuid)
}

async fn check_authentication<T: UserQuestionRepository + Repository, U: Authenticator, V: RedisService>(app_data: &Data<AppState<T, U, V>>, headers: Vec<(&str, &str)>) -> Result<i64, GenericResponse<()>> {
    match app_data.auth_service.get_user_id(headers, &app_data.auth_keys.decoding_key).await {
        Ok(user_id) => {
            if user_id == 0 {
                return Err(GenericResponse {
                    success: false,
                    data: None,
                    error: Some(GenericError {
                        code: 401,
                        entity: "ONE_EDU_APP".to_string(),
                        message: "Unauthorized".to_string()
                    })
                });
            }
            Ok(user_id)
        },
        Err(_) => {
            Err(GenericResponse {
                success: false,
                data: None,
                error: Some(GenericError {
                    code: 500,
                    entity: "ONE_EDU_APP".to_string(),
                    message: "Authentication error".to_string()
                })
            })
        }
    }
}

pub async fn get_exam<
    T: UserQuestionRepository + Repository,
    U: Authenticator,
    V: RedisService
>(
    app_data: Data<AppState<T, U, V>>, uuid: Uuid) -> Result<GenericResponse<UserQuestion>, GenericResponse<()>> {

    app_data.repo.get_user_question_by_uuid(uuid).await.map(|data| {
        GenericResponse {
            success: true,
            data: Some(data),
            error: None
        }
    }).map_err(|e| {
        GenericResponse {
            success: false,
            data: None,
            error: Some(GenericError {
                code: 500,
                entity: "ONE_EDU_APP".to_string(),
                message: e.to_string()
            })
        }
    })
}
