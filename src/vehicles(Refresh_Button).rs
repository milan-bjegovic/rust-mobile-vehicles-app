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
    let vehicles = use_signal(|| None::<Vec<Vehicle>>);
    let fetch_vehicles = move || {
        let mut vehicles = vehicles.clone();
        async move {
            let client = Client::new();
            if let Ok(response) = client.get("http://192.168.1.65:8080/vehicles/list").send().await {
                if let Ok(text) = response.text().await {
                    if let Ok(data) = serde_json::from_str::<Vec<Vehicle>>(&text) {
                        vehicles.set(Some(data));
                    }
                }
            }
        }
    };

    let _ = use_resource(move || async move {
        fetch_vehicles().await;
    });

    rsx! {
        div {
            class: "vehicle-container",
            button {
                onclick: move |_| fetch_vehicles(),
                "Refresh Vehicles"
            }
            match vehicles.read().as_ref() {
                Some(vehicles) => rsx! {
                    for vehicle in vehicles.iter() {
                        div { class: "vehicle-card",
                            img { src: "{vehicle.image_url}", class: "vehicle-image" }
                            h2 { "{vehicle.name}" }
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
