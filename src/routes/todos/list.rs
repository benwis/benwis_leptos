use leptos::*;
use leptos_router::*;
use crate::functions::todo::{get_todos, AddTodo, DeleteTodo};


#[component]
pub fn Todos(cx: Scope) -> impl IntoView {
    let add_todo = create_server_multi_action::<AddTodo>(cx);
    let delete_todo = create_server_action::<DeleteTodo>(cx);
    let submissions = add_todo.submissions();

    // list of todos is loaded from the server in reaction to changes
    let todos = create_resource(
        cx,
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(cx),
    );

    view! {
        cx,
        <div>
            <MultiActionForm action=add_todo>
                <label>
                    "Add a Todo"
                    <input type="text" name="title"/>
                </label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
            <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
                {move || {
                    let existing_todos = {
                        move || {
                            todos.read(cx)
                                .map(move |todos| match todos {
                                    Err(e) => {
                                        vec![view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_any()]
                                    }
                                    Ok(todos) => {
                                        if todos.is_empty() {
                                            vec![view! { cx, <p>"No tasks were found."</p> }.into_any()]
                                        } else {
                                            todos
                                                .into_iter()
                                                .map(move |todo| {
                                                    view! {
                                                        cx,
                                                        <li>
                                                            {todo.title}
                                                            ": Created at "
                                                            {todo.created_at}
                                                            " by "
                                                            {
                                                                todo.user.unwrap_or_default().username
                                                            }
                                                            <ActionForm action=delete_todo>
                                                                <input type="hidden" name="id" value={todo.id}/>
                                                                <input type="submit" value="X"/>
                                                            </ActionForm>
                                                        </li>
                                                    }
                                                    .into_any()
                                                })
                                                .collect::<Vec<_>>()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };

                    let pending_todos = move || {
                        submissions
                        .get()
                        .into_iter()
                        .filter(|submission| submission.pending().get())
                        .map(|submission| {
                            view! {
                                cx,
                                <li class="pending">{move || submission.input.get().map(|data| data.title) }</li>
                            }
                        })
                        .collect::<Vec<_>>()
                    };

                    view! {
                        cx,
                        <ul>
                            {existing_todos}
                            {pending_todos}
                        </ul>
                    }
                }
            }
            </Transition>
        </div>
    }
}