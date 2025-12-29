mod app;
mod config;
mod ui;
mod window;

use app::AuraLauncherApp;

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn")
    ).init();

    // Create and run the application
    let app = AuraLauncherApp::new();
    std::process::exit(app.run());
}
