use uuid::Uuid;

pub struct NewUser {
    pub uuid: Uuid,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    // pub university: String,
    // pub major: String,
    pub role: String,
    pub password: String
}
