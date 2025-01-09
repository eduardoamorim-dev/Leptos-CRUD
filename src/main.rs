use leptos::*;
mod components;
mod models;
use components::todo_list::TodoList;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="app-container">
            <h1>"CRUD com Leptos"</h1>
            <TodoList/>
        </main>
    }
}