use crate::models::accounts::{Task, Todo};
use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn TodoContainer() -> Element {
    let mut user_public_key = use_signal(|| "".to_string());
    let mut error_message = use_signal(|| "".to_string());
    let mut fetched_todos = use_signal(|| Option::<Vec<Todo>>::None);

    rsx! {
      div { class: "container",
          h1 { "Solana To-Do List Viewer" }
          p { "Enter a Solana public key to view the associated to-do list from the backend." }

          div { class: "input-group",
              // Input field for the public key
              // You'll bind this to your `user_pubkey` signal's `oninput` handler
              input {
                  r#type: "text", // Use r# to escape keyword "type"
                  placeholder: "Enter Solana Public Key",
                  value: "{user_public_key}",
                  oninput: move |event| user_public_key.set(event.value()), // Example: How you'd handle input
              },
              // Button to trigger the fetch operation
              // You'll bind this to your fetch logic in the `onclick` handler
              button {
                  onclick: move |_| async move {
                      let pub_key = user_public_key.read().clone();

                      on_ask_ai(&pub_key, &mut error_message, &mut fetched_todos).await;
                  },
                  "Fetch To-Do"
              }
          }

          if !error_message.read().is_empty() {
            p {
              class: "error-message",
              "{error_message.read()}"
            }
          }

          if let Some(todos) = fetched_todos.read().as_ref() {
            if todos.is_empty() {
              p { "Nothing to show here!!" }
            } else if error_message.read().is_empty() {
              ul {
                for todo in todos.iter() {
                  li {
                    class: "task-item",
                    span { class: "task-content", "{todo.content}" }
                    span { class: "task-status", "{todo.is_completed}" }
                  }
                }
              }
            }
          }
      }
    }
}

async fn on_ask_ai(
    provided_pub_key: &str,
    error_message: &mut Signal<String>,
    fetched_todos: &mut Signal<Option<Vec<Todo>>>,
) {
    if provided_pub_key.is_empty() {
        tracing::error!("No user public key provided");
    }

    let get_url = format!("http://localhost:3000/sol/{}", provided_pub_key);

    match reqwest::get(get_url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Task>().await {
                    Ok(response_json) => {
                        tracing::info!("Prepared response: {:?}", response_json);
                        error_message.set("".to_string());
                        fetched_todos.set(Some(response_json.todos));
                    }
                    Err(e) => {
                        tracing::error!("Response parsing issue: {:?}", e);
                        error_message.set(format!("Response parsing issue: {:?}", e));
                    }
                }
            } else {
                error_message.set(format!("Provided invalid public key"));
            }
        }
        Err(err) => {
            tracing::error!("Issue: {}", err);
            error_message.set(format!("Internal server error: {:?}", err));
        }
    }
}
