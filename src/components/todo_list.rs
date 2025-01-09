use leptos::*;
use uuid::Uuid;
use gloo_storage::{LocalStorage, Storage};
use crate::models::todo::Todo;
use leptos::ev::SubmitEvent;

#[component]
pub fn TodoList() -> impl IntoView {
    let (todos, set_todos) = create_signal(vec![]);
    let (new_todo_title, set_new_todo_title) = create_signal(String::new());

    // Carrega todos do localStorage
    create_effect(move |_| {
        if let Ok(stored_todos) = LocalStorage::get::<Vec<Todo>>("todos") {
            set_todos.set(stored_todos);
        }
    });

    // Salva todos no localStorage
    create_effect(move |_| {
        let _ = LocalStorage::set("todos", &todos.get());
    });

    let add_todo = move |ev: SubmitEvent| {
        ev.prevent_default();
        let title = new_todo_title.get();
        
        if !title.is_empty() {
            set_todos.update(|todos| {
                todos.push(Todo {
                    id: Uuid::new_v4().to_string(),
                    title,
                    completed: false,
                });
            });
            set_new_todo_title.set(String::new());
        }
    };

    let delete_todo = move |id: String| {
        set_todos.update(|todos| {
            todos.retain(|todo| todo.id != id);
        });
    };

    let toggle_todo = move |id: String| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    };

    view! {
        <div class="todo-container">
            <form on:submit=add_todo class="todo-form">
                <input
                    type="text"
                    placeholder="Adicionar nova tarefa"
                    prop:value=new_todo_title
                    on:input=move |ev| {
                        let input = event_target_value(&ev);
                        set_new_todo_title.set(input);
                    }
                    class="todo-input"
                />
                <button type="submit" class="todo-button">
                    "Adicionar Tarefa"
                </button>
            </form>

            <ul class="todo-list">
                {move || todos.get().into_iter().map(|todo| {
                    let todo_id = todo.id.clone();
                    let delete_id = todo_id.clone();
                    let toggle_id = todo_id.clone();
                    
                    view! {
                        <li class="todo-item">
                            <input
                                type="checkbox"
                                prop:checked=todo.completed
                                on:change=move |_| toggle_todo(toggle_id.clone())
                                class="todo-checkbox"
                            />
                            <span class=move || {
                                if todo.completed { "completed" } else { "" }
                            }>
                                {todo.title}
                            </span>
                            <button
                                on:click=move |_| delete_todo(delete_id.clone())
                                class="todo-delete"
                            >
                                "Deletar"
                            </button>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}