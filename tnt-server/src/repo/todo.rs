use crate::repo::RepoErr;
use crate::repo::RepoSpaces;
use front_app::Todo;
use tarantool::error::Error as TrtlErr;
use tarantool::index::IteratorType;
use tarantool::transaction::transaction;

/// Insert `todo` for the provided title. Empty title not allowed.
/// Returns `Result` with inserted Todo on success, otherwise
/// returns an `repo error`.
pub fn create_todo(title: &str) -> Result<Todo, RepoErr> {
    use RepoErr::EmptyTodoTitle;

    if title.is_empty() {
        return Err(EmptyTodoTitle);
    }
    let space_todo = RepoSpaces::Todo.find()?;
    let todo = space_todo
        .insert(&(None::<u32>, title, false))?
        .decode::<Todo>()?;

    Ok(todo)
}

/// Delete `todo` for the provided id.
/// Returns `Result` with deleted Todo on success, otherwise
/// returns an `repo error`.
pub fn delete_todo(id: u32) -> Result<Todo, RepoErr> {
    let space_todo = RepoSpaces::Todo.find()?;

    let todo = space_todo.delete(&[id])?.unwrap().decode::<Todo>()?;

    Ok(todo)
}

/// List all `todo`s.
/// Returns `Result` with Todos on success, otherwise
/// returns an `repo error`.
pub fn list_todos() -> Result<Vec<Todo>, RepoErr> {
    let space_todo = RepoSpaces::Todo.find()?;

    let todos = space_todo
        .select(IteratorType::All, &())?
        .map(|t| t.decode::<Todo>().unwrap())
        .collect();

    Ok(todos)
}

/// Change `completed` status of selected todo.
/// Returns `Result` with Todo on success, otherwise
/// returns an `repo error`.
pub fn change_completed(id: u32, completed: bool) -> Result<Todo, RepoErr> {
    let space_todo = RepoSpaces::Todo.find()?;

    let todo = space_todo
        .update(&[id], [("=", 2, completed)])?
        .unwrap()
        .decode::<Todo>()?;

    Ok(todo)
}

/// Change `completed` status of all todos.
/// Returns `Result` with Todos on success, otherwise
/// returns an `repo error`.
pub fn change_all_completed(completed: bool) -> Result<Vec<Todo>, RepoErr> {
    let space_todo = RepoSpaces::Todo.find()?;

    let res = transaction(|| -> Result<Vec<Todo>, TrtlErr> {
        let todos: Vec<Todo> = space_todo
            .select(IteratorType::All, &())?
            .map(|t| t.decode::<Todo>().unwrap())
            .collect();

        let mut res = Vec::<Todo>::with_capacity(todos.len());
        for todo in todos {
            let new_todo = space_todo
                .update(&[todo.id], [("=", 2, completed)])?
                .unwrap()
                .decode::<Todo>()
                .unwrap();
            res.push(new_todo);
        }

        Ok(res)
    })?;

    Ok(res)
}

/// Delete `completed` todos.
/// Returns `Result` with non completed Todos on success, otherwise
/// returns an `repo error`.
pub fn delete_completed() -> Result<Vec<Todo>, RepoErr> {
    let space_todo = RepoSpaces::Todo.find()?;

    let res = transaction(|| -> Result<Vec<Todo>, TrtlErr> {
        let todos: Vec<Todo> = space_todo
            .select(IteratorType::All, &())?
            .map(|t| t.decode::<Todo>().unwrap())
            .collect();

        for todo in todos.iter().filter(|t| t.completed) {
            space_todo.delete(&[todo.id])?;
        }

        let todos: Vec<Todo> = todos.iter().filter(|t| !t.completed).cloned().collect();

        Ok(todos)
    })?;

    Ok(res)
}

/// Delete `completed` todos.
/// Returns `Result` with non completed Todos on success, otherwise
/// returns an `repo error`.
pub fn change_title(id: u32, title: &str) -> Result<Todo, RepoErr> {
    let space_todo = RepoSpaces::Todo.find()?;

    let todo = space_todo
        .update(&[id], [("=", 1, title)])?
        .unwrap()
        .decode::<Todo>()
        .unwrap();

    Ok(todo)
}

#[cfg(feature = "test")]
mod tests {
    use super::*;

    #[tarantool_test::test]
    fn all_todos() {
        let expected: Vec<Todo> = (1..4)
            .map(|i| format!("New Todo {i:?}"))
            .map(|t| create_todo(&t).unwrap())
            .collect();
        let result = list_todos().unwrap();
        assert_eq!(result, expected);
    }

    #[tarantool_test::test]
    fn insert_todo() {
        let title = "New Todo 3";
        let result = create_todo(title).unwrap();
        let expected = Todo {
            id: result.id,
            title: title.to_string(),
            completed: false,
        };
        assert_eq!(result, expected);
    }

    #[tarantool_test::test]
    fn insert_same_data() {
        let title = "New Todo 4";
        create_todo(title).unwrap();
        create_todo(title).unwrap();
    }

    #[tarantool_test::test]
    fn insert_empty_title() {
        let title = "";
        let result = create_todo(title);
        assert_eq!(result, Err(RepoErr::EmptyTodoTitle));
    }

    #[tarantool_test::test]
    fn destroy_todo() {
        let title = "New Todo 5";
        let result = create_todo(title).unwrap();
        let expected = Todo {
            id: result.id,
            title: title.to_string(),
            completed: false,
        };
        let result = delete_todo(result.id).unwrap();
        assert_eq!(result, expected);
    }

    #[tarantool_test::test]
    fn change_completed_status() {
        let todo = create_todo("New Todo 6").unwrap();
        let result = change_completed(todo.id, true).unwrap();
        assert_eq!(result.completed, true);
    }

    #[tarantool_test::test]
    fn activate_all_todo() {
        let result = change_all_completed(false).unwrap();
        assert!(result.iter().all(|t| t.completed));
    }

    #[tarantool_test::test]
    fn complete_all_todo() {
        let result = change_all_completed(true).unwrap();
        assert!(result.iter().all(|t| t.completed));
    }

    #[tarantool_test::test]
    fn delete_completed_todos() {
        let result = delete_completed().unwrap();
        assert_eq!(result, Vec::new());
    }

    #[tarantool_test::test]
    fn change_todo_title() {
        let todo = create_todo("New Todo 1").unwrap();
        let new_title = "New Todo 2";
        let result = change_title(todo.id, new_title).unwrap();
        assert_eq!(
            result,
            Todo {
                id: todo.id,
                title: new_title.to_string(),
                completed: false
            }
        );
    }
}
