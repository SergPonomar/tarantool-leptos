use crate::Todo;
use leptos::ev::{Event, KeyboardEvent};
use leptos::html::Input;
use leptos::*;

#[component]
pub fn TodoItem<CC, CT, D>(
    todo: Memo<Todo>,
    on_change_completed: CC,
    on_change_title: CT,
    on_destroy: D,
) -> impl IntoView
where
    CC: Fn(u32, bool) + 'static,
    CT: Fn(u32, String) + 'static,
    D: Fn(u32) + 'static + Clone,
{
    let on_toggle = move |ev: Event| {
        let new_value = event_target_checked(&ev);
        on_change_completed(todo.with(|t| t.id), new_value);
    };

    let editing_input = create_node_ref::<Input>();
    let (editing, set_editing) = create_signal(false);
    let (input, set_input) = create_signal(todo.with(|t| t.title.clone()));

    let start_editing = move |_| {
        set_input.set(todo.with(|t| t.title.clone()));
        set_editing.set(true);
        let _ = editing_input.get().unwrap().focus();
    };

    let destroy_fn = on_destroy.clone();
    let finish_editing = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            if input.get().trim().is_empty() {
                destroy_fn(todo.with(|t| t.id));
            } else {
                on_change_title(todo.with(|t| t.id), input.get())
            };
            set_editing.set(false);
        }
    };

    let cancel_editing = move |_| {
        set_editing.set(false);
    };

    view! {
        <li
            class:completed=move || todo.with(|t| t.completed)
            class:editing=editing
        >
            <div class="view">
                <input
                    type="checkbox"
                    class="toggle"
                    prop:checked=move || todo.with(|t| t.completed)
                    on:change=on_toggle
                />
                <label on:dblclick=start_editing>{ move || todo.with(|t| t.title.clone()) }</label>
                <button
                    class="destroy"
                    on:click=move |_| on_destroy(todo.with(|t| t.id))
                ></button>
            </div>
            <div class="input-container">
                <input
                    id="edit-todo-input"
                    _ref=editing_input
                    type="text"
                    class="edit"
                    on:keyup=finish_editing
                    on:blur=cancel_editing
                    on:input=move |ev| { set_input.set(event_target_value(&ev))}
                    prop:value=input
                />
                <label class="visually-hidden" for="edit-todo-input">Edit Todo Input</label>
            </div>
        </li>
    }
}
