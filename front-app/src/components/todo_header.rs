use leptos::ev::KeyboardEvent;
use leptos::*;
use leptos_router::A;

#[component]
pub fn TodoHeader<F>(on_add_todo: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let (input, set_input) = create_signal(String::new());

    let on_key_app = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            on_add_todo(input.get());
            set_input.update(|s| s.clear());
        }
    };

    view! {
        <header class="header">
            <A href="/"><h1>todos</h1></A>
            <input
                type="text"
                class="new-todo"
                autofocus
                autocomplete="off"
                placeholder="What needs to be done?"
                on:keyup=on_key_app
                on:input=move |ev| { set_input.set(event_target_value(&ev))}
                prop:value=input
            />
      </header>
    }
}
