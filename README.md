# VehiclesApp – A Secure, Fullstack, Cross-Platform Application for Vehicle Management

VehiclesApp is a robust, fullstack Rust application designed to provide a modern, high-performance, and secure solution for vehicle management across multiple platforms — including web, desktop, mobile (Android/iOS), and server-side environments. Powered by the cutting-edge Dioxus UI framework and asynchronous programming through Tokio, VehiclesApp represents a modular, scalable, and secure architecture suitable for both personal and enterprise-level vehicle-related applications.

### 🔧 Technical Overview

✅ Core Technologies
Rust 2021 Edition: Provides memory safety, speed, and zero-cost abstractions for high performance and low-level control.

Dioxus (v0.6): A React-like UI framework for Rust used to build the frontend across web, desktop, and mobile platforms using a single codebase.

Features used: router, fullstack, web, desktop, mobile, and server.

Tokio (v1.43): A powerful asynchronous runtime used to manage concurrent tasks efficiently, especially useful in network requests or background processing.

Serde & Serde JSON: Reliable data serialization/deserialization for managing structured vehicle data across client-server communications or local storage.

### 🔐 Security & Encryption

AES encryption with aes, block-modes, and block-padding crates allows for secure handling of sensitive data such as vehicle ownership details, authentication tokens, or location history.

The inclusion of cryptographic tools positions the app for potential secure offline storage or end-to-end encrypted communication between clients and servers.

### 🎯 Multi-Platform Feature Flags

web: Compiles to WASM using dioxus/web for a modern browser-based interface.

desktop: Native application using dioxus/desktop, suitable for laptops and PCs.

mobile: Targets Android/iOS with dioxus/mobile, ideal for users on the go.

server: Backend support using dioxus/server, enabling the same components and logic to be reused on the server side.

### 🔄 Profiles for Development

Custom profiles such as wasm-dev, server-dev, and android-dev enable efficient development and optimization for each target environment, with tailored compilation and optimization settings.

### 🚀 Potential Use Cases

VehiclesApp is highly adaptable and can be repurposed or expanded to support a wide range of vehicle-related services, including but not limited to:

Vehicle Inventory Management: For dealerships or collectors to organize and present vehicles with details like make, model, VIN, price, and availability.

Vehicle Auctions: With encrypted bid handling, secure user authentication, and real-time updates.

Fleet Tracking Systems: With integration of real-time GPS data, maintenance logs, and driver profiles.

Classic Car Showcase App: Ideal for enthusiasts and clubs to share photos, specs, and histories of restored classic vehicles.

Rental and Leasing Applications: Supporting booking, contract management, and availability.

### 🧩 Extensibility & Future Enhancements

The commented-out dependencies in the Cargo.toml suggest future enhancements or modular expansions:

WebAssembly utilities (wasm-bindgen, web-sys): For deeper integration with the browser environment.

Tower HTTP with CORS support: For building REST APIs that interact securely across different domains.

Cross-platform directories (dirs): For storing user-specific settings or cache.

Base64 & JNI: Indicating plans for Android-native bindings or more advanced encoding capabilities.

These indicate a forward-thinking design ready to integrate advanced features such as biometric authentication, native Android functionality, or cloud synchronization.

### 🛠️ Developer-Friendly Architecture

VehiclesApp emphasizes modularity and developer productivity:

Profiles can be individually optimized for various targets.

Rust’s strong type system and error handling reduce runtime bugs.

Using Dioxus allows writing UI in idiomatic Rust without switching to JavaScript or React, while maintaining a reactive component model.

### 📫 Maintainer

Author: Milan Bjegovic
Email: milanbjegovic@gmail.com / oldtimersoffer@gmail.com 

### Tailwind

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform mobile
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

