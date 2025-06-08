use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
      style {
              ".container {{ max-width: 600px; margin: 40px auto; padding: 20px; font-family: Arial, sans-serif; box-shadow: 0 0 10px rgba(0,0,0,0.1); border-radius: 8px; }}"
              "h1 {{ color: #333; text-align: center; }}"
              "p {{ color: #555; text-align: center; }}"
              ".input-group {{ display: flex; margin-bottom: 20px; }}"
              "input[type='text'] {{ flex-grow: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px 0 0 4px; font-size: 16px; }}"
              "button {{ padding: 10px 15px; background-color: #007bff; color: white; border: none; border-radius: 0 4px 4px 0; font-size: 16px; cursor: pointer; }}"
              "button:hover {{ background-color: #0056b3; }}"
              ".task-list {{ list-style-type: none; padding: 0; }}"
              ".task-item {{ background-color: #f9f9f9; border: 1px solid #eee; padding: 10px; margin-bottom: 8px; border-radius: 4px; display: flex; align-items: center; }}"
              ".task-item.completed .task-content {{ text-decoration: line-through; color: #888; }}"
              ".task-status {{ margin-right: 10px; font-size: 1.2em; }}"
              ".task-content {{ flex-grow: 1; }}"
              ".error-message {{ color: #D8000C; background-color: #FFD2D2; border: 1px solid #D8000C; padding: 10px; margin-top: 15px; border-radius: 4px; text-align: center; }}"
              ".loading-message, .no-tasks-message {{ text-align: center; color: #777; margin-top: 20px; }}"
          }

          div { class: "container",
              h1 { "Solana To-Do List Viewer" }
              p { "Enter a Solana public key to view the associated to-do list from the backend." }

              div { class: "input-group",
                  // Input field for the public key
                  // You'll bind this to your `user_pubkey` signal's `oninput` handler
                  input {
                      r#type: "text", // Use r# to escape keyword "type"
                      placeholder: "Enter Solana Public Key",
                      // value: "{user_pubkey}", // Example: How you'd bind value
                      // oninput: move |event| user_pubkey.set(event.value()), // Example: How you'd handle input
                  }
                  // Button to trigger the fetch operation
                  // You'll bind this to your fetch logic in the `onclick` handler
                  button {
                      // onclick: move |_| { /* your async fetch logic here */ },
                      "Fetch To-Dos"
                  }
              }
          }
    }
}
