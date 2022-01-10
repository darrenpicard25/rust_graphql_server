use std::sync::Arc;

use crate::repositories::user;

use super::entities::User;

pub struct Input {
    pub email: String,
    pub password: String,
}

pub enum RegisterError {
    AlreadyExists,
    Unknown,
}

pub async fn execute(repo: Arc<dyn user::Repository>, input: Input) -> Result<User, RegisterError> {
    let previous_user = repo.find_one().await;

    match previous_user {
        Err(user::FindOneError::NotFound) => {}
        Err(user::FindOneError::Unknown) => return Err(RegisterError::Unknown),
        Ok(_) => return Err(RegisterError::AlreadyExists),
    }

    Ok(User {
        email: input.email,
        password: input.password,
    })
}

#[cfg(test)]
mod tests {
    use crate::repositories::user::MockRepository;

    use super::*;

    #[tokio::test]
    async fn should_return_user() {
        let mut repo = MockRepository::new();
        repo.expect_find_one()
            .times(1)
            .returning(|| Err(user::FindOneError::NotFound));

        let email = "email".to_string();
        let password = "password".to_string();
        let results = execute(
            Arc::new(repo),
            Input {
                email: email.clone(),
                password: password.clone(),
            },
        )
        .await;

        match results {
            Ok(user) => assert_eq!(user, User { email, password }),
            _ => unreachable!(),
        }
    }
}
