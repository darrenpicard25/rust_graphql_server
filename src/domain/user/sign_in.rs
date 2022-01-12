use crate::repositories::user;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

use super::{entities::User, hash_password};

pub struct Input {
    pub email: String,
    pub password: String,
}
pub enum SignInError {
    InvalidPasswordFormat,
    Failed,
    Unknown,
}

async fn logic(repo: Arc<dyn user::Repository>, input: Input) -> Result<User, SignInError> {
    let Input { email, password } = input;

    let user = repo.find_one_by_email(email.clone()).await;

    let user = match user {
        Ok(Some(user)) => user,
        Ok(None) => return Err(SignInError::Failed),
        Err(user::FindOneByEmailError::Unknown) => return Err(SignInError::Unknown),
    };

    let hashed_password = match hash_password::execute(password) {
        Ok(pass) => pass,
        Err(_) => return Err(SignInError::InvalidPasswordFormat),
    };

    if user.password != hashed_password {
        return Err(SignInError::Failed);
    }

    Ok(User {
        id: user.id,
        email: user.email,
        password: user.password,
    })
}

pub async fn execute(repo: Arc<dyn user::Repository>, input: Input) -> Result<User, SignInError> {
    // Adding delay so it always take 500ms to respond to prevent from seeing difference
    let (_, results) = tokio::join!(sleep(Duration::from_millis(500)), logic(repo, input));

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_return_user_if_password_match() {
        let mut repo = user::MockRepository::new();
        repo.expect_find_one_by_email().times(1).returning(|email| {
            Ok(Some(User {
                id: "id".to_string(),
                email,
                password: hash_password::execute("pass".to_string()).unwrap(),
            }))
        });

        let results = execute(
            Arc::new(repo),
            Input {
                email: "email".to_string(),
                password: "pass".to_string(),
            },
        )
        .await;

        match results {
            Ok(_) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn should_return_failed_error_if_no_user_found() {
        let mut repo = user::MockRepository::new();
        repo.expect_find_one_by_email()
            .times(1)
            .returning(|_| Ok(None));

        let results = execute(
            Arc::new(repo),
            Input {
                email: "email".to_string(),
                password: "pass".to_string(),
            },
        )
        .await;

        match results {
            Err(SignInError::Failed) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn should_return_failed_error_if_password_doesnt_match() {
        let mut repo = user::MockRepository::new();
        repo.expect_find_one_by_email().times(1).returning(|email| {
            Ok(Some(User {
                id: "id".to_string(),
                email,
                password: "unknown".to_string(),
            }))
        });

        let results = execute(
            Arc::new(repo),
            Input {
                email: "email".to_string(),
                password: "pass".to_string(),
            },
        )
        .await;

        match results {
            Err(SignInError::Failed) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn should_return_unknown_error_if_repo_return_unknown() {
        let mut repo = user::MockRepository::new();
        repo.expect_find_one_by_email()
            .times(1)
            .returning(|_| Err(user::FindOneByEmailError::Unknown));

        let results = execute(
            Arc::new(repo),
            Input {
                email: "email".to_string(),
                password: "pass".to_string(),
            },
        )
        .await;

        match results {
            Err(SignInError::Unknown) => {}
            _ => unreachable!(),
        }
    }
}
