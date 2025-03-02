use async_trait::async_trait;
use log::error;
use sqlx::error::Error;
use sqlx::{query_as};
use crate::commons::authentication::password_hash::{hash_password, verify_password};
use crate::commons::repositories::base::{ConnGetter, DbRepo, EntityId, UserLoginResponse};
use crate::commons::repositories::users::user_model::NewUser;

#[async_trait]
pub trait UserRepository {
    async fn check_user_existence(&self, username: String, email: String, phone_number: String, partner: String) -> Result<bool, Error>;
    async fn check_user_by_phone_number(&self, phone_number: String, partner: String) -> Result<bool, Error>;
    async fn check_user_by_email(&self, email: String, partner: String) -> Result<bool, Error>;
    async fn check_user_by_username(&self, username: String, partner: String) -> Result<bool, Error>;
    async fn register_user(&self, new_user: NewUser) -> Result<EntityId, Error>;
    async fn login_via_user_name(&self, username: String, password: String, partner: String) -> Result<UserLoginResponse, Error>;
    async fn login_via_phone_number(&self, phone_number: String, password: String, partner: String) -> Result<UserLoginResponse, Error>;
    async fn login_via_email(&self, email: String, password: String, partner: String) -> Result<UserLoginResponse, Error>;
}

#[async_trait]
impl UserRepository for DbRepo {
    async fn check_user_existence(&self, username: String, email: String, phone_number: String, partner: String) -> Result<bool, Error> {
        let result = query_as::<_, EntityId>(
            r"
                select id
                from users
                where partner = $1 and (user_name = $2 or email = $3 or phone_number = $4)
                ")
            .bind(&partner)
            .bind(&username)
            .bind(&email)
            .bind(&phone_number)
            .fetch_one(self.get_conn())
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    async fn check_user_by_phone_number(&self, phone_number: String, partner: String) -> Result<bool, Error> {
        let result = query_as::<_, EntityId>(
            r"
                select id
                from users
                where partner = $1 and phone_number = $2
                ")
            .bind(&partner)
            .bind(&phone_number)
            .fetch_one(self.get_conn())
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    async fn check_user_by_email(&self, email: String, partner: String) -> Result<bool, Error> {
        let result = query_as::<_, EntityId>(
            r"
                select id
                from users
                where partner = $1 and email = $2
                ")
            .bind(&partner)
            .bind(&email)
            .fetch_one(self.get_conn())
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    async fn check_user_by_username(&self, username: String, partner: String) -> Result<bool, Error> {
        let result = query_as::<_, EntityId>(
            r"
                select id
                from users
                where partner = $1 and user_name = $2
                ")
            .bind(&partner)
            .bind(&username)
            .fetch_one(self.get_conn())
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    async fn register_user(&self, new_user: NewUser) -> Result<EntityId, Error> {
        let mut tx = self.get_conn().begin().await.unwrap();

        // todo: need to test min password length 8 and max 200
        let hashed_password = hash_password(&new_user.password).unwrap();
        let insert_result = query_as::<_, EntityId>(
            r"
            insert into users
            (uuid, user_name, full_name, email, phone_number, role, password, partner)
            values
            ($1, $2, $3, $4, $5, $6, $7, $8)
            returning id
            ")
            .bind(new_user.uuid)
            .bind(new_user.username)
            .bind(new_user.full_name.clone())
            .bind(new_user.email.clone())
            .bind(new_user.phone_number)
            .bind(new_user.role)
            .bind(hashed_password)
            .bind(new_user.partner)
            .fetch_one(&mut *tx)
            .await;

        let inserted_entity = match insert_result {
            Ok(row) => Ok(row),
            Err(e) => {
                error!("create user error: {:?}", e);
                Err(e)
            }
        };

        if let Err(e) = inserted_entity {
            return Err(e);
        }

        _ = tx.commit().await;

        Ok(EntityId { id: inserted_entity?.id })
    }

    async fn login_via_user_name(&self, username: String, password: String, partner: String) -> Result<UserLoginResponse, Error> {
        let result = query_as::<_, UserLoginResponse>(
            r"
            select id, user_name, email, password
            from users
            where user_name = $1 and partner = $2
            ")
            .bind(&username)
            .bind(&partner)
            .fetch_one(self.get_conn())
            .await;

        match result {
            Ok(row) => {
                if verify_password(&password, &row.password).unwrap() {
                    Ok(row)
                } else {
                    Err(Error::Protocol("Invalid password".into()))
                }
            },
            Err(e) => {
                error!("login error: {:?}", e);
                Err(e)
            }
        }
    }

    async fn login_via_phone_number(&self, phone_number: String, password: String, partner: String) -> Result<UserLoginResponse, Error> {
        let result = query_as::<_, UserLoginResponse>(
            r"
            select id, user_name, email, password
            from users
            where phone_number = $1 and partner = $2
            ")
            .bind(&phone_number)
            .bind(&partner)
            .fetch_one(self.get_conn())
            .await;

        match result {
            Ok(row) => {
                if verify_password(&password, &row.password).unwrap() {
                    Ok(row)
                } else {
                    Err(Error::Protocol("Invalid password".into()))
                }
            },
            Err(e) => {
                error!("login error: {:?}", e);
                Err(e)
            }
        }
    }

    async fn login_via_email(&self, email: String, password: String, partner: String) -> Result<UserLoginResponse, Error> {
        let result = query_as::<_, UserLoginResponse>(
            r"
            select id, user_name, email, password
            from users
            where email = $1 and partner = $2
            ")
            .bind(&email)
            .bind(&partner)
            .fetch_one(self.get_conn())
            .await;

        match result {
            Ok(row) => {
                if verify_password(&password, &row.password).unwrap() {
                    Ok(row)
                } else {
                    Err(Error::Protocol("Invalid password".into()))
                }
            },
            Err(e) => {
                error!("login error: {:?}", e);
                Err(e)
            }
        }
    }
}
