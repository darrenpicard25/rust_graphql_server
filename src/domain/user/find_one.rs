use std::sync::Arc;

use crate::repositories::user;

use super::entities::User;

pub enum FindOneError {
    Unknown,
    InvalidId,
    NotFound,
}

pub async fn execute(repo: Arc<dyn user::Repository>, id: String) -> Result<User, FindOneError> {
    let result = repo.find_by_id(id).await;

    match result {
        Ok(user) => Ok(User {
            id: user.id,
            email: user.email,
            password: user.password,
        }),
        Err(user::FindByIdError::NotFound) => Err(FindOneError::NotFound),
        Err(user::FindByIdError::InvalidId) => Err(FindOneError::InvalidId),
        Err(user::FindByIdError::Unknown) => Err(FindOneError::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::user::MockRepository;

    use super::*;

    #[tokio::test]
    async fn should_return_user() {
        let stub_user = User {
            id: "id".to_string(),
            email: "email".to_string(),
            password: "password".to_string(),
        };
        let stub_user_2 = stub_user.clone();
        let mut repo = MockRepository::new();
        repo.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(stub_user_2.clone()));

        let results = execute(Arc::new(repo), stub_user.id.clone()).await;

        match results {
            Ok(user) => assert_eq!(user, stub_user),
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn should_return_error_if_no_user_found() {
        let mut repo = MockRepository::new();
        repo.expect_find_by_id()
            .times(1)
            .returning(|_| Err(user::FindByIdError::NotFound));

        let results = execute(Arc::new(repo), "id".to_string()).await;

        match results {
            Err(FindOneError::NotFound) => {}
            _ => unreachable!(),
        }
    }
}
