// Stringr -- Powerful text editor built with Rust and Dioxus
// Main entry point

// Allow unused code warnings - we have defined theme constants and APIs for future use
#![allow(dead_code)]

mod app;
mod theme;
mod editor;
mod document;
mod file;
mod ui;

fn main() {
    // Init logging
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Stringr...");

    // Launch Dioxus desktop app with window configuration
    dioxus::LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title("Stringr")
                        .with_resizable(true)
                )
        )
        .launch(app::app);
}