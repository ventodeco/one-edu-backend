use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::error;
use sqlx::{query, query_as, Error};
use uuid::Uuid;
use crate::commons::repositories::base::{ConnGetter, DbRepo};
use crate::commons::repositories::user_questions::user_question_model::{QuestionDetails, UserQuestion};

#[async_trait]
pub trait UserQuestionRepository {
    async fn get_user_question_by_uuid(&self, uuid: Uuid) -> Result<UserQuestion, Error>;
    async fn update_data_for_start_exam(&self, uuid: Uuid, started_time: DateTime<Utc>, ended_time: DateTime<Utc>, question_details: QuestionDetails) -> Result<(), Error>;
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

    async fn update_data_for_start_exam(&self, uuid: Uuid, started_time: DateTime<Utc>, ended_time: DateTime<Utc>, question_details: QuestionDetails) -> Result<(), Error> {
        let question_details_json = serde_json::to_value(&question_details);

        let result = query(
            r"
            update user_questions
            set
                status = 'IN_PROGRESS',
                started_time = $1,
                ended_time = $2,
                question_details = $3
            where uuid = $4::UUID
            ")
            .bind(started_time)
            .bind(ended_time)
            .bind(question_details_json.unwrap())
            .bind(uuid)
            .execute(self.get_conn())
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("update data to start exam error: {:?}", e);
                Err(e)
            }
        }
    }
}