use crate::repo::todo::{
    change_all_completed, change_completed, change_title, create_todo, delete_completed,
    delete_todo, list_todos,
};
use axum::body::Body;
use axum::extract::{RawQuery, State};
use axum::http::header::{self, HeaderMap};
use axum::http::Request;
use axum::response::Response;
use axum::Router;
use axum::{extract::Path as ExtractPath, http::StatusCode, response::IntoResponse, routing::get};
use core::time::Duration;
use front_app::*;
use leptos::*;
use leptos_axum::{
    generate_route_list, handle_server_fns_with_context, render_app_to_stream, LeptosRoutes,
};

use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;
use tarantool::fiber;
use tarantool::space::{FieldType, Space};

mod repo;
mod test;

use repo::RepoSpaces;

/// Custom axum handler for leptos server functions
async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: ExtractPath<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move || {
            provide_context(app_state.cmd_tx.clone());
            provide_context(app_state.todo_rx.clone());
        },
        request,
    )
    .await
}

/// Custom axum handler for leptos server functions
async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<Body>) -> Response {
    let handler = render_app_to_stream(app_state.leptos_options.clone(), || view! { <App/> });
    handler(req).await.into_response()
}

/// Tarantool entry point (main function)
#[tarantool::proc]
fn start() {
    create_spaces();
    let _ = read_files();
    let (cmd_tx, cmd_rx) = mpsc::channel();
    let (todo_tx, todo_rx) = mpsc::channel::<Vec<Todo>>();
    let todo_rx = Arc::new(Mutex::new(todo_rx));

    let jh = std::thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(async move {
                let conf = get_configuration(Some("./Cargo.toml")).await.unwrap();
                let leptos_options = conf.leptos_options;
                let addr = leptos_options.site_addr.clone();
                let shared_state = AppState {
                    todo_rx,
                    cmd_tx,
                    leptos_options,
                };

                let routes = generate_route_list(App);

                let app = Router::new()
                    .route("/pkg/:path", get(file_server))
                    .route(
                        "/api/*fn_name",
                        get(server_fn_handler).post(server_fn_handler),
                    )
                    .leptos_routes_with_handler(routes, get(leptos_routes_handler))
                    .with_state(shared_state);

                println!("listening on http://{}", &addr);
                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .await
                    .unwrap();

                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            })
    });

    drop(jh);

    static mut FIBER_JOIN_HANDLE: Option<fiber::JoinHandle<()>> = None;
    let jh = fiber::start(move || loop {
        match cmd_rx.try_recv() {
            Err(TryRecvError::Empty) => {
                fiber::sleep(Duration::from_millis(10));
                continue;
            }
            Err(TryRecvError::Disconnected) => break,
            Ok(Cmd::GetTodos) => {
                if let Ok(todos) = list_todos() {
                    let _ = todo_tx.send(todos);
                }
            }
            Ok(Cmd::AddTodo(title)) => {
                let opes = move || {
                    create_todo(&title)?;
                    list_todos()
                };
                if let Ok(todos) = opes() {
                    let _ = todo_tx.send(todos);
                }
            }
            Ok(Cmd::DeleteTodo(id)) => {
                let opes = move || {
                    delete_todo(id)?;
                    list_todos()
                };
                if let Ok(todos) = opes() {
                    let _ = todo_tx.send(todos);
                }
            }
            Ok(Cmd::ChangeTitle((id, title))) => {
                let opes = move || {
                    change_title(id, &title)?;
                    list_todos()
                };
                if let Ok(todos) = opes() {
                    let _ = todo_tx.send(todos);
                }
            }
            Ok(Cmd::ChangeAllCompleted(completed)) => {
                if let Ok(todos) = change_all_completed(completed) {
                    let _ = todo_tx.send(todos);
                }
            }
            Ok(Cmd::ChangeCompleted((id, completed))) => {
                let opes = move || {
                    change_completed(id, completed)?;
                    list_todos()
                };
                if let Ok(todos) = opes() {
                    let _ = todo_tx.send(todos);
                }
            }
            Ok(Cmd::DeleteCompleted) => {
                if let Ok(todos) = delete_completed() {
                    let _ = todo_tx.send(todos);
                }
            }
        }
    });

    unsafe {
        FIBER_JOIN_HANDLE = Some(jh);
    }
}

/// Create tarantool spaces
fn create_spaces() {
    for space in RepoSpaces::iter() {
        let _ = space.create();
    }

    let files_space = Space::builder("files")
        .format([("name", FieldType::String), ("content", FieldType::Any)])
        .if_not_exists(true)
        .create()
        .unwrap();

    files_space
        .index_builder("files_idx")
        .part("name")
        .if_not_exists(true)
        .create()
        .unwrap();
}

/// Serve application files on /pkg route
async fn file_server(ExtractPath(path): ExtractPath<String>) -> impl IntoResponse {
    let file_space = Space::find_cached("files").unwrap();
    let ext = path.split('.').last();
    let mime;
    match ext {
        Some("wasm") => {
            mime = "application/wasm";
        }
        Some("js") => {
            mime = "application/javascript";
        }
        Some("css") => {
            mime = "text/css";
        }
        _ => return Err(StatusCode::NOT_FOUND),
    }

    if let Ok(Some(tuple)) = file_space.get(&(path,)) {
        if let Ok(Some(file)) = tuple.field::<Vec<u8>>(1) {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, mime.parse().unwrap());
            return Ok((headers, file));
        }
    }

    Err(StatusCode::NOT_FOUND)
}

/// Read application files and store in `files` space
fn read_files() -> io::Result<()> {
    let space_name = Space::find_cached("files").unwrap();
    let dir = Path::new("pkg");

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // Pass only application files
            if !path
                .extension()
                .is_some_and(|ext| ["wasm", "js", "css"].contains(&ext.to_str().unwrap()))
            {
                continue;
            }
            let file = File::open(path)?;
            let file_name = entry.file_name();
            let mut buf_reader = BufReader::new(file);
            let mut contents = Vec::new();
            buf_reader.read_to_end(&mut contents)?;

            let _res = space_name.insert(&(&file_name.into_string().unwrap(), contents));
        }
    }
    Ok(())
}
