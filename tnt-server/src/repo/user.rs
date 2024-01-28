use crate::repo::RepoErr;
use crate::repo::RepoSpaces;

#[derive(Debug, PartialEq)]
pub struct User {
    id: u32,
    name: String,
}

/// Insert `user` for the provided name. Empty name not allowed.
/// Returns `Result` with inserted User on success, otherwise
/// returns an `repo error`.
#[allow(dead_code)]
pub fn create_user(name: &str) -> Result<User, RepoErr> {
    use RepoErr::{EmptyUserName, FieldNotExsist};

    if name.is_empty() {
        return Err(EmptyUserName);
    }
    let space_user = RepoSpaces::User.find()?;
    let todo = space_user.insert(&(None::<u32>, name))?;

    let id = todo.get(0).ok_or(FieldNotExsist(String::from("id")))?;
    let name = todo.get(1).ok_or(FieldNotExsist(String::from("name")))?;

    Ok(User { id, name })
}

#[cfg(feature = "test")]
mod tests {
    use super::*;

    #[tarantool_test::test]
    fn insert_user() {
        let name = "New user 1";
        let result = create_user(name).unwrap();
        let expected = User {
            id: result.id,
            name: name.to_string(),
        };
        assert_eq!(result, expected);
    }

    #[tarantool_test::test]
    fn insert_same_data() {
        let name = "New user 2";
        create_user(name).unwrap();
        let result = create_user(name);
        assert!(result.is_err());
    }

    #[tarantool_test::test]
    fn insert_empty_name() {
        let name = "";
        let result = create_user(name);
        assert_eq!(result, Err(RepoErr::EmptyUserName));
    }
}
