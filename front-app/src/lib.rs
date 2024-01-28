mod components;

use crate::components::HomePage;
#[cfg(feature = "ssr")]
use axum::extract::FromRef;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(FromRef))]
pub struct AppState {
    pub cmd_tx: Sender<Cmd>,
    pub todo_rx: Arc<Mutex<Receiver<Vec<Todo>>>>,
    pub leptos_options: LeptosOptions,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Cmd {
    GetTodos,
    AddTodo(String),
    DeleteTodo(u32),
    ChangeTitle((u32, String)),
    ChangeCompleted((u32, bool)),
    ChangeAllCompleted(bool),
    DeleteCompleted,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (todos, set_todos) = create_signal(Vec::<Todo>::new());

    #[cfg(not(feature = "ssr"))]
    spawn_local(async move {
        let new_todos = get_todos().await.unwrap();
        set_todos.set(new_todos);
    });

    view! {
        <Stylesheet href="/pkg/index.css"/>
        <Stylesheet href="/pkg/base.css"/>

        <Title text="Simple tarantool app"/>

        <Router>
            <section class="todoapp">
                <Routes>
                    <Route path="/" view=move || {
                        view! {
                            <HomePage todos=todos set_todos=set_todos/>
                        }
                    }/>
                    <Route path="/active" view=move || {
                        view! {
                            <HomePage todos=todos set_todos=set_todos/>
                        }
                    }/>
                    <Route path="/completed" view=move || {
                        view! {
                            <HomePage todos=todos set_todos=set_todos/>
                        }
                    }/>
                </Routes>
            </section>
        </Router>
        <footer class="info">
            <p>Double-click to edit a todo</p>
            <p>Created by the TodoMVC Team</p>
            <p>Part of <a href="http://todomvc.com">TodoMVC</a></p>
        </footer>
    }
}

pub fn cmd_tx() -> Result<Sender<Cmd>, ServerFnError> {
    use_context::<Sender<Cmd>>()
        .ok_or_else(|| ServerFnError::ServerError("Sender cmd missing.".into()))
}

pub fn todo_rx() -> Result<Arc<Mutex<Receiver<Vec<Todo>>>>, ServerFnError> {
    use_context::<Arc<Mutex<Receiver<Vec<Todo>>>>>()
        .ok_or_else(|| ServerFnError::ServerError("Receiver missing.".into()))
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let cmd_tx = cmd_tx()?;
    let _ = cmd_tx.send(Cmd::GetTodos);
    let todo_rx = todo_rx()?;
    let todos = todo_rx.lock().unwrap().recv().unwrap();
    Ok(todos)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<Vec<Todo>, ServerFnError> {
    let cmd_tx = cmd_tx()?;
    let _ = cmd_tx.send(Cmd::AddTodo(title));
    let todo_rx = todo_rx()?;
    let todos = todo_rx.lock().unwrap().recv().unwrap();
    Ok(todos)
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: u32) -> Result<Vec<Todo>, ServerFnError> {
    let cmd_tx = cmd_tx()?;
    let _ = cmd_tx.send(Cmd::DeleteTodo(id));
    let todo_rx = todo_rx()?;
    let todos = todo_rx.lock().unwrap().recv().unwrap();
    Ok(todos)
}

#[server(ChangeTitle, "/api")]
pub async fn change_title(id: u32, title: String) -> Result<Vec<Todo>, ServerFnError> {
    let cmd_tx = cmd_tx()?;
    let _ = cmd_tx.send(Cmd::ChangeTitle((id, title)));
    let todo_rx = todo_rx()?;
    let todos = todo_rx.lock().unwrap().recv().unwrap();
    Ok(todos)
}

#[server(ChangeCompleted, "/api")]
pub async fn change_completed(id: u32, completed: bool) -> Result<Vec<Todo>, ServerFnError> {
    let cmd_tx = cmd_tx()?;
    let _ = cmd_tx.send(Cmd::ChangeCompleted((id, completed)));
    let todo_rx = todo_rx()?;
    let todos = todo_rx.lock().unwrap().recv().unwrap();
    Ok(todos)
}

#[server(ChangeAllCompleted, "/api")]
pub async fn change_all_completed(completed: bool) -> Result<Vec<Todo>, ServerFnError> {
    let cmd_tx = cmd_tx()?;
    let _ = cmd_tx.send(Cmd::ChangeAllCompleted(completed));
    let todo_rx = todo_rx()?;
    let todos = todo_rx.lock().unwrap().recv().unwrap();
    Ok(todos)
}

#[server(DeleteCompleted, "/api")]
pub async fn delete_completed() -> Result<Vec<Todo>, ServerFnError> {
    let cmd_tx = cmd_tx()?;
    let _ = cmd_tx.send(Cmd::DeleteCompleted);
    let todo_rx = todo_rx()?;
    let todos = todo_rx.lock().unwrap().recv().unwrap();
    Ok(todos)
}
