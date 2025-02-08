

# Rust macOS Tray App

A native macOS menu‑bar (tray) application built in Rust. This project demonstrates how to integrate Rust with macOS’s native Cocoa frameworks via Objective‑C interop. It creates a tray icon using SF Symbols, implements a dynamic NSMenu with custom menu items (each with its own icon), and triggers notifications when menu items are selected. In addition, the README includes instructions for bundling your app as a proper macOS application so that notifications and other system features work as expected.

---

## Table of Contents

- [Rust macOS Tray App](#rust-macos-tray-app)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Features](#features)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Building and Running](#building-and-running)
    - [Running in Development Mode](#running-in-development-mode)
  - [Bundling as a macOS App](#bundling-as-a-macos-app)
    - [Step 1: Install cargo-bundle](#step-1-install-cargo-bundle)
    - [Step 2: Configure Cargo.toml](#step-2-configure-cargotoml)
    - [Step 3: Bundle the Application](#step-3-bundle-the-application)
    - [Step 4: Run the Bundled App](#step-4-run-the-bundled-app)
  - [Troubleshooting](#troubleshooting)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)

---

## Overview

The **Rust macOS Tray App** is a proof‑of‑concept application that demonstrates how to build a native macOS tray (menu‑bar) application using Rust. The app:

- Uses Cocoa’s `NSStatusBar` to create a tray icon.
- Displays a dynamic menu (`NSMenu`) with custom items that include SF Symbol icons.
- Implements three menu items:
  - **Update Status:** Sends a notification indicating that the status was updated.
  - **Security Preferences:** Sends a notification indicating that security preferences are being opened.
  - **Quit:** Sends a quit notification and terminates the app.
- Sends notifications using the (deprecated) `NSUserNotificationCenter` API.
- Provides bundling instructions using `cargo-bundle` to package the project as a macOS app for proper system integration.

---

## Features

- **Native Tray Icon:**  
  Uses `NSStatusBar` to add a tray icon with the SF Symbol `"shield.fill"` that adapts to light and dark mode.

- **Custom NSMenu with Icons:**  
  The tray icon opens a menu with items:
  - **Update Status:** With the icon `"arrow.clockwise"`.
  - **Security Preferences:** With the icon `"gearshape.fill"`.
  - **Quit:** With the icon `"xmark.circle.fill"` and a custom action that terminates the app after sending a notification.

- **Notifications:**  
  Each custom menu item triggers a notification via the (deprecated) `NSUserNotificationCenter` API. (Note: For best results, see [Bundling as a macOS App](#bundling-as-a-macos-app) below.)

- **Bundling Support:**  
  Provides instructions for packaging your application as a macOS app bundle using [`cargo-bundle`](https://github.com/burtonageo/cargo-bundle) so that notifications and system integration work properly.

- **Rust & Cocoa Interop:**  
  Demonstrates using the `cocoa` and `objc` crates to call Objective‑C APIs from Rust.

---

## Requirements

- **Rust Toolchain:**  
  Ensure you have the latest stable version of Rust installed. [Install Rust via rustup](https://rustup.rs/).

- **macOS:**  
  This application is designed for macOS. It works on both Intel and Apple Silicon devices.

- **cargo-bundle (optional, for bundling):**  
  To package your app as a proper macOS app bundle, install `cargo-bundle`:
  ```bash
  cargo install cargo-bundle
  ```

---

## Installation

Clone the repository and change into the project directory:

```bash
git clone <repository-url>
cd tray
```

Make sure your **Cargo.toml** is configured properly (see the example below).

---

## Usage

When running the application, it adds an icon to the macOS menu bar. Clicking the tray icon reveals a menu with the following items:

- **Update Status:**  
  Sends a notification (e.g., “Status updated successfully.”).

- **Security Preferences:**  
  Sends a notification (e.g., “Opening Security Preferences...”).

- **Quit:**  
  Sends a notification and then terminates the application.

> **Note:** When running as a command‑line tool (using `cargo run`), macOS may not show notifications because the app isn’t bundled. For reliable notifications, please see the bundling instructions below.

---

## Building and Running

### Running in Development Mode

You can build and run the application directly with:

```bash
cargo run
```

Since this is a tray app, it will run in the background without a traditional window. To stop the app, select “Quit” from the tray menu or terminate it via the terminal.

---

## Bundling as a macOS App

Bundling the app creates a proper macOS application bundle, ensuring that notifications and other system features behave as expected.

### Step 1: Install cargo-bundle

If you haven’t already installed it, run:

```bash
cargo install cargo-bundle
```

### Step 2: Configure Cargo.toml

At the end of your **Cargo.toml**, add the following metadata:

```toml
[package.metadata.bundle]
identifier = "com.example.tray"
# Optionally specify an icon (must be in .icns format):
# icon = "resources/icon.icns"
```

### Step 3: Bundle the Application

Build the app bundle in release mode:

```bash
cargo bundle --release
```

This command creates a macOS app bundle (e.g., `tray.app`) in:

```
target/release/bundle/macos/tray.app
```

### Step 4: Run the Bundled App

Launch your app by running:

```bash
open target/release/bundle/macos/tray.app
```

Now, your app will run as a proper macOS application and system notifications should appear as expected when you interact with the menu.

---

## Troubleshooting

- **No Notifications:**  
  If you don’t see notifications when clicking “Update Status” or “Security Preferences…”:
  - Ensure you are running the bundled version of the app.
  - Check your macOS Notification settings to ensure notifications are allowed for your app.
  - Remember that the NSUserNotificationCenter API is deprecated; for a modern approach, consider using UNUserNotificationCenter (though that requires additional work).

- **Build Errors:**  
  Verify you have the latest Rust toolchain and that your dependencies (`cocoa`, `objc`) are up to date.

- **Icon Issues:**  
  If icons do not display properly, ensure your system supports SF Symbols (macOS 11 or later).

---

## Contributing

Contributions are welcome! If you encounter issues or have ideas for improvements, please open an issue or submit a pull request. Whether it’s enhancing the notification logic, updating to a modern notification API, or improving the overall structure, your help is appreciated.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Acknowledgements

- **Cocoa and objc crates:**  
  These crates make it possible to integrate Rust with macOS’s native frameworks.
- **The Rust community:**  
  For the ongoing development of tools and libraries that enable cross‑platform native development.
- **macOS Documentation:**  
  For detailed information on Cocoa, NSStatusBar, NSMenu, and notifications.

---

Enjoy building native macOS apps in Rust! If you have questions or suggestions, please feel free to contribute or reach out via the project’s repository.

Happy coding!

