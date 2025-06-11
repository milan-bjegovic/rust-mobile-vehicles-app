use dioxus::prelude::*;
use dioxus::prelude::ServerFnError;
use serde::{Deserialize, Serialize};
use crate::server_fn::request::reqwest::Client;
use crate::token::{encrypt_and_save_token, get_storage_path, load_and_decrypt_token};


// Define the data structure for the login request
#[derive(Serialize, Deserialize, Clone, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    session_code: String,
}


#[component]
pub fn LoginForm() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut result = use_signal(|| None::<Result<String, ServerFnError>>);
    
    rsx! {
        form {
            onsubmit: move |_| {
                let credentials = LoginRequest {
                    username: username.read().clone(),
                    password: password.read().clone(),
                };
                spawn(async move {
                    let client = Client::new();
                    let response = client
                        .post("http://192.168.1.65:8080/users/login")
                        .header("Content-Type", "application/json")
                        .body(serde_json::to_string(&credentials).unwrap()) // Convert to JSON string
                        .send()
                        .await;

                    match response {
                        Ok(resp) => {
                            let text = resp.text().await.unwrap_or_else(|_| "No response".to_string());
                            result.set(Some(Ok(text.clone())));
                            let _token = encrypt_and_save_token(&text);

                            println!("Key: {:?}", load_and_decrypt_token());
                            println!("Key path: {:?}", get_key_path());
                            println!("Storage path: {:?}", get_storage_path());
                        }
                        Err(e) => result.set(Some(Err(dioxus::prelude::ServerFnError::Response(e.to_string())))),
                    }
                });
            },
            input {
                value: "{username}",
                oninput: move |e| username.set(e.value().clone()),
                placeholder: "Username"
            }
            input {
                value: "{password}",
                oninput: move |e| password.set(e.value().clone()),
                placeholder: "Password",
                r#type: "password"
            }
            button { r#type: "submit", "Login" }
        }
        if let Some(res) = result.read().as_ref() {
            match res {
                Ok(msg) => rsx!("{msg}"), // Directly render the message
                Err(e) => rsx!("{e}"),   // Directly render the error
            }
        }
    }
}