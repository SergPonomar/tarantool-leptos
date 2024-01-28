use crate::components::{TodoFooter, TodoHeader, TodoItem};
use crate::{
    add_todo, change_all_completed, change_completed, change_title, delete_completed, delete_todo,
    Todo,
};
use leptos::ev::Event;
use leptos::*;
use leptos_router::use_location;

#[component]
pub fn HomePage(todos: ReadSignal<Vec<Todo>>, set_todos: WriteSignal<Vec<Todo>>) -> impl IntoView {
    let (toggle_all, set_toggle_all) = create_signal(false);
    let remaining = move || todos.with(|ts| ts.iter().filter(|t| !t.completed).count());
    let completed = move || todos.with(|ts| ts.iter().filter(|t| t.completed).count());

    let pathname = use_location().pathname;

    let filter = match pathname.get().as_str() {
        "/" => |_t: &Todo| true,
        "/active" => |t: &Todo| !t.completed,
        "/completed" => |t: &Todo| t.completed,
        &_ => unreachable!(),
    };

    let filter_todos = move || {
        todos
            .get()
            .into_iter()
            .filter(|todo| filter(todo))
            .collect::<Vec<Todo>>()
    };

    let on_toggle = move |ev: Event| {
        let new_value = event_target_checked(&ev);
        set_toggle_all.set(new_value);
        spawn_local(async move {
            let new_todos = change_all_completed(new_value).await.unwrap();
            set_todos.set(new_todos);
        });
    };

    let on_add_todo = move |s| {
        spawn_local(async move {
            let new_todos = add_todo(s).await.unwrap();
            set_todos.set(new_todos);
        });
    };

    let on_destroy = move |id| {
        spawn_local(async move {
            let new_todos = delete_todo(id).await.unwrap();
            set_todos.set(new_todos);
        });
    };

    let on_change_title = move |id, title| {
        spawn_local(async move {
            let new_todos = change_title(id, title).await.unwrap();
            set_todos.set(new_todos);
        });
    };

    let on_change_completed = move |id, completed| {
        spawn_local(async move {
            let new_todos = change_completed(id, completed).await.unwrap();
            set_todos.set(new_todos);
        });
    };

    let on_delete_completed = move || {
        spawn_local(async move {
            let new_todos = delete_completed().await.unwrap();
            set_todos.set(new_todos);
        });
    };

    view! {
        <TodoHeader on_add_todo=on_add_todo />
        <Show
            when=move || todos.with(|t| !t.is_empty())
        >
            <main class="main">
                <div class="toggle-all-container">
                    <input
                        type="checkbox"
                        id="toggle-all-input"
                        class="toggle-all"
                        prop:checked=toggle_all
                        prop:disabled=todos.with(|ts| {
                            ts.is_empty() || (!toggle_all.get() && !ts.iter().any(|t| !t.completed))
                        })
                        on:change=on_toggle
                    />
                    <label class="toggle-all-label" htmlFor="toggle-all-input"> Toggle All Input </label>
                </div>
                <ul class="todo-list">
                    <For
                        each=move || filter_todos()
                        key=|todo| todo.id
                        children=move |todo| {
                            let memo_todo = create_memo(move |_| {
                                todos.with(|todos| todos.into_iter().find(|t| t.id == todo.id).unwrap_or(&todo).clone())
                            });

                            view! {
                                <TodoItem
                                    todo=memo_todo
                                    on_change_completed=on_change_completed
                                    on_change_title=on_change_title
                                    on_destroy=on_destroy
                                />
                            }
                        }
                    />
                </ul>
            </main>
            <TodoFooter
                remaining=remaining
                completed=completed
                on_delete_completed=on_delete_completed
            />
        </Show>
    }
}
