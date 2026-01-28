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
mod shortcuts;

fn main() {
    // Force X11 backend on Linux for reliable window event handling
    // (Wayland has timing issues with wry/tao close events)
    #[cfg(target_os = "linux")]
    {
        if std::env::var("GDK_BACKEND").is_err() {
            std::env::set_var("GDK_BACKEND", "x11");
        }
    }

    // Init logging
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Stringr...");

    // Launch Dioxus desktop app with window configuration
    // We disable native decorations to use custom themed window chrome
    dioxus::LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(
                    dioxus::desktop::WindowBuilder::new()
                        .with_title("Stringr")
                        .with_resizable(true)
                        .with_decorations(false) // Custom window chrome
                )
                // Use LastWindowExitsApp for clean exit behavior
                .with_close_behaviour(dioxus::desktop::WindowCloseBehaviour::LastWindowExitsApp)
                // Set a themed background color to prevent flash
                .with_background_color((0x14, 0x14, 0x19, 0xFF))
        )
        .launch(app::app);
}