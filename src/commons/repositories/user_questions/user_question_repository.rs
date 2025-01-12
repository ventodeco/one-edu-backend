use async_trait::async_trait;
use log::error;
use sqlx::{query_as, Error};
use uuid::Uuid;
use crate::commons::repositories::base::{ConnGetter, DbRepo};
use crate::commons::repositories::user_questions::user_question_model::UserQuestion;

#[async_trait]
pub trait UserQuestionRepository {
    async fn get_user_question_by_uuid(&self, uuid: Uuid) -> Result<UserQuestion, Error>;
}

#[async_trait]
impl UserQuestionRepository for DbRepo {
    async fn get_user_question_by_uuid(&self, uuid: Uuid) -> Result<UserQuestion, Error> {

        let result = query_as::<_, UserQuestion>(
            r"
            select uuid, user_id, exam_uuid, status, question_details, time_limit, started_time, ended_time, created_at, updated_at
            from user_questions
            where uuid = $1::UUID
            ")
            .bind(uuid)
            .fetch_one(self.get_conn())
            .await;

        match result {
            Ok(row) => Ok(row),
            Err(e) => {
                error!("get user question by uuid error: {:?}", e);
                Err(e)
            }
        }
    }
}