use dioxus::prelude::*;
use serde::Deserialize;
use serde_json;

// Use your custom Client
use crate::server_fn::request::reqwest::Client;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct Vehicle {
    name: String,
    description: String,
    starting_price: String,
    image_url: String,
}

#[component]
pub fn VehicleList() -> Element {
    let vehicles = use_resource(|| async move {
        let client = Client::new();
        let response = client.get("http://192.168.1.65:8080/vehicles/list")
            .send()
            .await
            .unwrap();  // Replace with error handling
        let text = response.text().await.unwrap();  // Replace with error handling
        serde_json::from_str::<Vec<Vehicle>>(&text).unwrap()  // Replace with error handling
    });

    rsx! {
        div {
            class: "vehicle-container",
            match vehicles.read().as_ref() {
                Some(vehicles) => rsx! {
                    for vehicle in vehicles.iter() {
                        div { class: "vehicle-card",
                            img { src: "{vehicle.image_url}", class: "vehicle-image" }
                            h2 { "{vehicle.name}" }
                            // p { "{vehicle.description}" }
                            div { dangerous_inner_html: "{vehicle.description}" }
                            p { "Starting Price: ${vehicle.starting_price}" }
                        }
                    }
                },
                None => rsx! { "Loading vehicles..." },
            }
        }
    }
}