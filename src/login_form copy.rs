use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::server_fn::request::reqwest::Client;

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
    let mut result = use_signal(|| None::<Result<String, String>>); // Use String for error

    let handle_login = move |_| {
        let credentials = LoginRequest {
            username: username.read().clone(),
            password: password.read().clone(),
        };

        spawn(async move {
            let client = Client::new();
            let response = client
                .post("http://0.0.0.0:8080/users/login")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&credentials).unwrap())
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.text().await {
                            Ok(login_response) => {
                                result.set(Some(Ok(login_response))); // Corrected line
                            }
                            Err(e) => {
                                result.set(Some(Err(format!("Failed to parse response: {}", e))));
                            }
                        }
                    } else {
                        result.set(Some(Err(format!(
                            "Login failed: {}",
                            resp.status()
                        ))));
                    }
                }
                Err(e) => result.set(Some(Err(format!("Network error: {}", e)))),
            }
        });
    };

    rsx! {
        form {
            onsubmit: handle_login,
            input {
                value: "{username}",
                oninput: move |e| username.set(e.value().clone()),
                placeholder: "Username",
            }
            input {
                value: "{password}",
                oninput: move |e| password.set(e.value().clone()),
                placeholder: "Password",
                r#type: "password",
            }
            button { r#type: "submit", "Login" }
        }

        if let Some(res) = result.read().as_ref() {
            match res {
                Ok(_msg) => rsx! { p { "Login successful" } },
                Err(e) => rsx! { p { "Error: {e}" } },
            }
        }
    }
}