use dioxus::{logger::tracing, prelude::*};
use reqwest::Client;

#[component]
pub fn TodoContainer() -> Element {
    let mut user_public_key = use_signal(|| "".to_string());

    tracing::info!("{user_public_key}");

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
              }
              // Button to trigger the fetch operation
              // You'll bind this to your fetch logic in the `onclick` handler
              button {
                  onclick: move |_| async move {
                    if user_public_key.read().is_empty() {
                      tracing::error!("No user public key provided");
                    }

                    let get_url = format!("http://localhost:3000/sol/{}", user_public_key.read());

                    match reqwest::get(get_url).await {
                      Ok(response) => {
                        tracing::info!("{:?}", response);
                      },
                      Err(err) => {
                        tracing::error!("Issue: {}", err);
                      }
                    }
                  },
                  "Fetch To-Do"
              }
          }
      }
    }
}
