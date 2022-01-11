use std::sync::Arc;

use crate::repositories::user;

use super::entities::User;

pub struct Input {
    pub email: String,
    pub password: String,
}
#[derive(PartialEq, Eq, Debug)]
pub enum RegisterError {
    AlreadyExists,
    Unknown,
}

pub async fn execute(repo: Arc<dyn user::Repository>, input: Input) -> Result<User, RegisterError> {
    let Input { email, password } = input;
    let previous_user = repo.find_one_by_email(email.clone()).await;

    match previous_user {
        Ok(None) => {}
        Ok(Some(_)) => return Err(RegisterError::AlreadyExists),
        Err(user::FindOneByEmailError::Unknown) => return Err(RegisterError::Unknown),
    };

    let results = repo.create(user::CreateInput { email, password }).await;

    match results {
        Ok(user) => Ok(User {
            id: user.id,
            email: user.email,
            password: user.password,
        }),
        Err(user::CreateError::Unknown) => Err(RegisterError::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::user::MockRepository;

    use super::*;

    #[tokio::test]
    async fn should_return_user() {
        let mut repo = MockRepository::new();
        repo.expect_find_one_by_email()
            .times(1)
            .returning(|_| Ok(None));

        repo.expect_create()
            .times(1)
            .returning(|user::CreateInput { email, password }| {
                Ok(User {
                    id: "id".to_string(),
                    email,
                    password,
                })
            });

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
            Ok(user) => assert_eq!(
                user,
                User {
                    id: "id".to_string(),
                    email,
                    password
                }
            ),
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn should_return_already_exists_error_when_already_found() {
        let mut repo = MockRepository::new();
        let email = "email".to_string();
        let password = "password".to_string();
        repo.expect_find_one_by_email().times(1).returning(|email| {
            Ok(Some(User {
                id: "id".to_string(),
                email,
                password: "pass".to_string(),
            }))
        });

        let results = execute(
            Arc::new(repo),
            Input {
                email: email.clone(),
                password: password.clone(),
            },
        )
        .await;

        match results {
            Err(error) => assert_eq!(error, RegisterError::AlreadyExists),
            _ => unreachable!(),
        }
    }
}
