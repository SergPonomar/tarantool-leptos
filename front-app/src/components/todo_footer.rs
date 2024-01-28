use leptos::*;
use leptos_router::A;

#[component]
pub fn TodoFooter<R, C, DC>(remaining: R, completed: C, on_delete_completed: DC) -> impl IntoView
where
    R: Fn() -> usize + 'static,
    C: Fn() -> usize + 'static,
    DC: Fn() -> () + 'static + Clone,
{
    view! {
        <footer class="footer">
            <span class="todo-count">
                <strong>{ remaining() }</strong>" "{ if remaining() == 1 { "item" } else { "items" } }" left"
            </span>
            <ul class="filters">
                <li><A href="/">All</A></li>
                <li><A href="/active">Active</A></li>
                <li><A href="/completed">Completed</A></li>
            </ul>
                <button
                    on:click=move |_| on_delete_completed()
                    prop:disabled=move || { completed() == 0 }
                    class="clear-completed"
                >Clear Completed</button>
        </footer>
    }
}
