use std::sync::Arc;

use crate::repositories::user;

use super::entities::User;

pub struct Input {
    pub email: String,
    pub password: String,
}
pub enum SignInError {
    Failed,
    Unknown,
}

pub async fn execute(repo: Arc<dyn user::Repository>, input: Input) -> Result<User, SignInError> {
    let Input { email, password } = input;
    let user = repo.find_one_by_email(email.clone()).await;

    let user = match user {
        Ok(Some(user)) => user,
        Ok(None) => return Err(SignInError::Failed),
        Err(user::FindOneByEmailError::Unknown) => return Err(SignInError::Unknown),
    };

    if user.password != password {
        return Err(SignInError::Failed);
    }

    Ok(User {
        id: user.id,
        email: user.email,
        password: user.password,
    })
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
                password: "pass".to_string(),
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
