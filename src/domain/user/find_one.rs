use super::entities::User;

pub fn execute() -> User {
    User {
        email: "email".to_string(),
        password: "pass".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_user() {
        let results = execute();

        assert_eq!(
            results,
            User {
                email: "email".to_string(),
                password: "pass".to_string(),
            }
        )
    }
}
