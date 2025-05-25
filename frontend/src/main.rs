// src/main.rs
#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App);
}

// Our main application component
fn App() -> Element {
    // --- State Management with Signals ---

    // A signal to hold the text in the input field.
    // We start with an empty String.
    let mut prompt = use_signal(String::new);

    // A signal to hold the latest response from our AI backend.
    // We start with a placeholder message.
    let ai_response = use_signal(|| "The AI's response will appear here...".to_string());

    // A signal to track if we are currently waiting for a response.
    // This will be used to disable the input and button during a request.
    let is_loading = use_signal(|| false);

    // --- UI Rendering ---
    rsx! {
        // A container for our entire app with some basic styling
        div {
            style: "
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                gap: 1rem;
                padding: 2rem;
                font-family: sans-serif;
                max-width: 600px;
                margin: auto;
                background-color: #f4f7f6;
                border-radius: 10px;
            ",
            h1 { "Rusty ChatGPT" }

            // The response area
            p {
                style: "
                    background-color: #e8f0fe;
                    padding: 1rem;
                    border-radius: 8px;
                    min-height: 50px;
                    width: 100%;
                    white-space: pre-wrap; // To respect newlines in the response
                ",
                // The text of this paragraph is directly tied to our `ai_response` signal.
                "{ai_response}"
            }

            // A form to contain our input and button
            form {
                // We prevent the default form submission behavior (which reloads the page)
                // using the onsubmit event handler.
                onsubmit: move |event| {
                    // TODO: Trigger the AI request here
                    event.prevent_default();
                    println!("Form submitted with prompt: {}", prompt.read());
                },
                style: "width: 100%; display: flex; gap: 0.5rem;",
                // The text input field
                input {
                    // The `value` is bound to our `prompt` signal.
                    value: "{prompt}",
                    placeholder: "Enter your prompt...",
                    disabled: is_loading(), // Disable the input when loading
                    style: "flex-grow: 1; padding: 0.5rem; border-radius: 5px; border: 1px solid #ccc;",
                    // The `oninput` event fires every time the user types.
                    oninput: move |event| {
                        // We `set` the signal's value to the new input value.
                        // This updates our state.
                        prompt.set(event.value());
                    }
                }
                // The submit button
                button {
                    r#type: "submit", // Make it a submit button for the form
                    disabled: is_loading(), // Disable the button when loading
                    style: "padding: 0.5rem 1rem; border-radius: 5px; border: none; background-color: #007bff; color: white; cursor: pointer;",
                    if is_loading() { "Thinking..." } else { "Ask" }
                }
            }
        }
    }
}
