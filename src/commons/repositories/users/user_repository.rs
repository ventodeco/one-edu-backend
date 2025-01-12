use async_trait::async_trait;
use log::error;
use sqlx::error::Error;
use sqlx::{query_as};
use crate::commons::authentication::password_hash::{hash_password, verify_password};
use crate::commons::repositories::base::{ConnGetter, DbRepo, EntityId, UserLoginResponse};
use crate::commons::repositories::users::user_model::NewUser;

#[async_trait]
pub trait UserRepository {
    async fn register_user(&self, new_user: NewUser) -> Result<EntityId, Error>;
    async fn login(&self, username: String, password: String) -> Result<UserLoginResponse, Error>;
}

#[async_trait]
impl UserRepository for DbRepo {
    async fn register_user(&self, new_user: NewUser) -> Result<EntityId, Error> {
        let mut tx = self.get_conn().begin().await.unwrap();

        // todo: need to test min password length 8 and max 200
        let hashed_password = hash_password(&new_user.password).unwrap();
        let insert_result = query_as::<_, EntityId>(
            r"
            insert into users
            (uuid, user_name, full_name, email, phone_number, role, password)
            values
            ($1, $2, $3, $4, $5, $6, $7)
            returning id
            ")
            .bind(new_user.uuid)
            .bind(new_user.username)
            .bind(new_user.full_name.clone())
            .bind(new_user.email.clone())
            .bind(new_user.phone_number)
            .bind(new_user.role)
            .bind(hashed_password)
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

    async fn login(&self, email: String, password: String) -> Result<UserLoginResponse, Error> {
        let result = query_as::<_, UserLoginResponse>(
            r"
            select id, user_name, email, password
            from users
            where email = $1
            ")
            .bind(&email)
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
