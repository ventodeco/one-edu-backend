use std::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow, Postgres, Type};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use uuid::Uuid;

#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserQuestion {
    pub uuid: Uuid,
    pub user_id: i64,
    pub exam_uuid: String,
    pub status: String,
    pub question_details: QuestionDetails,
    pub time_limit: Option<i32>,
    pub started_time: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_time: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuestionDetails {
    // pub uuid: Uuid,
    pub answered_questions: Option<i32>,
    pub answered_question_list: Option<Vec<i32>>,
    pub total_questions: i32,
    pub title: String,
    pub description: Option<String>,
    pub time_limit: i32,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub questions: Vec<Question>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub number: i32,
    pub review: String,
    pub question: String,
    pub image_url: Option<String>,
    pub reference: Option<String>,
    pub answer_list: Vec<Answer>,
    pub correct_answer: String,
    pub answered: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Answer {
    pub key: String,
    pub value: String,
}

impl<'r> Decode<'r, Postgres> for QuestionDetails {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let value_str = value.as_str()?;
        let cleaned_value_str = value_str.trim_start_matches(|c| c < ' '); // Remove any control characters at the start
        serde_json::from_str(cleaned_value_str).map_err(|e| {
            format!("Failed to decode QuestionDetails from JSON: {}. JSON: {}", e, cleaned_value_str).into()
        })
    }
}

impl<'r> Encode<'r, Postgres> for QuestionDetails {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Box<dyn Error + Send + Sync>> {
        match serde_json::to_string(self) {
            Ok(json) => {
                buf.extend_from_slice(json.as_bytes());
                Ok(IsNull::No)
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl Type<Postgres> for QuestionDetails {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("jsonb")
    }
}
