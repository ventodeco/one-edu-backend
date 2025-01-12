use actix_web::web::Data;
use log::info;
use uuid::Uuid;
use crate::app_state::AppState;
use crate::commons::authentication::auth_keys_service::Authenticator;
use crate::commons::repositories::base::Repository;
use crate::commons::repositories::user_questions::user_question_model::{UserQuestion};
use crate::commons::repositories::user_questions::user_question_repository::UserQuestionRepository;
use crate::services::authentications::authentication_dto::{GenericError, GenericResponse};
use crate::services::redis::redis_service::RedisService;
use crate::services::user_questions::user_question_dto::{GetExamSummaryData, StartExamResponse};

pub async fn get_exam_summary<
    T: UserQuestionRepository + Repository,
    U: Authenticator,
    V: RedisService
>(
    app_data: Data<AppState<T, U, V>>, req: actix_web::HttpRequest, uuid: Uuid) -> Result<GenericResponse<GetExamSummaryData>, GenericResponse<GetExamSummaryData>> {

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
                entity: "testing".to_string(),
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

async fn check_authentication<T: UserQuestionRepository + Repository, U: Authenticator, V: RedisService>(app_data: &Data<AppState<T, U, V>>, headers: Vec<(&str, &str)>) -> Result<i64, GenericResponse<GetExamSummaryData>> {
    match app_data.auth_service.get_user_id(headers, &app_data.auth_keys.decoding_key).await {
        Ok(user_id) => {
            if user_id == 0 {
                return Err(GenericResponse {
                    success: false,
                    data: None,
                    error: Some(GenericError {
                        code: 401,
                        entity: "testing".to_string(),
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
                    entity: "testing".to_string(),
                    message: "Authentication error".to_string()
                })
            })
        }
    }
}

pub async fn start_exam<
    T: UserQuestionRepository + Repository,
    U: Authenticator,
    V: RedisService
>(
    app_data: Data<AppState<T, U, V>>, uuid: Uuid) -> Result<GenericResponse<UserQuestion>, GenericResponse<StartExamResponse>> {

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
                entity: "testing".to_string(),
                message: e.to_string()
            })
        }
    })
}
