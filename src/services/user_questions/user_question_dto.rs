use serde::{Serialize};
use uuid::Uuid;
use crate::commons::repositories::user_questions::user_question_model::UserQuestion;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartExamResponse {
    pub data: UserQuestion
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExamSummaryData {
    pub uuid: Uuid,
    pub title: String,
    pub description: String,
    pub total_questions: i32,
    pub time_limit: i32,
}
