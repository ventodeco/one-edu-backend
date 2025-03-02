use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use actix_http::body::BoxBody;
use actix_web::{HttpResponse, Responder, ResponseError};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub old_token: String,
    pub dev_or_emp: RouteDeveloperOrEmployer
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPassword {
    pub email: String,
    pub dev_or_emp: RouteDeveloperOrEmployer
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouteResetPassword {    
    pub user_id: i64,
    pub new_password: String,
    pub dev_or_emp: RouteDeveloperOrEmployer,
    pub unique_key: Uuid
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub user_identifier: String,
    pub password: String,
    pub partner: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub partner: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub access_token: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {
    pub id: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericError {
    pub code: i32,
    pub entity: String,
    pub message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<GenericError>
}

impl<T: Serialize> Responder for GenericResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize OutputId")
        }
    }
}

impl<T> Debug for GenericResponse<T> {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<T> Display for GenericResponse<T> {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<T: Serialize> ResponseError for GenericResponse<T> {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(self)
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum RouteDeveloperOrEmployer {
    Developer = 0,
    Employer = 1
}

#[derive(PartialEq, Debug)]
pub enum AuthenticateResult {
    Success{ id: i64 },
    Failure
}
