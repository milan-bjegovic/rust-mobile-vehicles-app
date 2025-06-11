use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::{server_fn::request::reqwest::Client, token::load_token_from_file};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CreateVehicle {
    name: String,
    description: String,
    starting_price: f64,
    image: String,
}

pub fn create_vehicle_form() -> Element {
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut starting_price = use_signal(|| 0.0);
    let mut image = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());

    let on_submit = move |event: FormEvent| {
        event.prevent_default();

        let vehicle_data = CreateVehicle {
            name: name.read().clone(),
            description: description.read().clone(),
            starting_price: starting_price.read().clone(),
            image: image.read().clone(),
        };

        println!("vehicle: {:?}", vehicle_data);

        spawn(async move {
            match load_token_from_file() {
                Some(token) => {
                
                let client = Client::new();

                let _res = client.post("http://192.168.1.65:8080/vehicles/create")
                    .header("Content-Type", "application/json")
                    .header("Session-Code", token.trim())
                    .body(serde_json::to_string(&vehicle_data).unwrap())
                    .send()
                    .await;

                    message.set("Vehicle added.".to_string());

            }
            None => {
                // Set the message to a plain String, not an Option or Result
                message.set("User is not authenticated. Please log in.".to_string());
            }
        }});
    };

    rsx! {
        form {
            onsubmit: on_submit,
            div {
                label { "Name" }
                input {
                    r#type: "text",
                    value: "{name}",
                    oninput: move |e| name.set(e.value().clone())
                }
            }
            div {
                label { "Description" }
                textarea {
                    value: "{description}",
                    oninput: move |e| description.set(e.value().clone())
                }
            }
            div {
                label { "Starting Price" }
                input {
                    r#type: "number",
                    step: "0.01",
                    value: "{starting_price}",
                    oninput: move |e| {
                        if let Ok(value) = e.value().parse::<f64>() {
                            starting_price.set(value);
                        }
                    }
                }
            }
            div {
                label { "Image URL" }
                input {
                    r#type: "text",
                    value: "{image}",
                    oninput: move |e| image.set(e.value().clone())
                }
            }
            button { "Create Vehicle" }
        }
        // Optional: Display the message
        div { "{message}" }
    }
}