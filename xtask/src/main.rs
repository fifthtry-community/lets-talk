use std::env;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), String> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("build-wasm") => fastn_xtask::build_wasm().map_err(|e| e.to_string())?,
        Some("run-template") => fastn_xtask::run_template().map_err(|e| e.to_string())?,
        Some("optimise-wasm") => fastn_xtask::optimise_wasm().map_err(|e| e.to_string())?,
        Some("publish-app") => fastn_xtask::publish_app().map_err(|e| e.to_string())?,
        Some("update-ui") => fastn_xtask::update_ui().map_err(|e| e.to_string())?,
        Some("run-ui") => fastn_xtask::run_ui().map_err(|e| e.to_string())?,
        Some("update-www") => fastn_xtask::update_www().map_err(|e| e.to_string())?,
        Some("run-www") => fastn_xtask::run_www().map_err(|e| e.to_string())?,
        Some("update-template") => fastn_xtask::update_template().map_err(|e| e.to_string())?,
        _ => print_help(),
    }

    Ok(())
}

fn print_help() {
    eprintln!(
        r#"fastn xtask CLI

USAGE:
    cargo xtask <COMMAND>

COMMANDS:
    build-wasm: Builds the WASM target from backend.

    run-ui: Builds and serves the UI for the lets-XXX app, which is served on port 8002.

    update-ui: Updates UI dependencies for the lets-XXX app, run this only when modifying dependencies in lets-XXX.fifthtry.site/FASTN.ftd or during the initial setup.

    run-template: Runs the backend and tests end-to-end functionality of the lets-XXX app.

    update-template: Updates dependencies for the lets-XXX app's backend template. Run this only when modifying dependencies or during the initial setup.

    run-www: Serves and tests the public website for the lets-XXX app.

    update-www: Updates dependencies for the lets-XXX app's public website. Run this only when modifying dependencies or during the initial setup.

    optimise-wasm: Optimises the generated WASM binary.

    publish-app: Publishes the lets-XXX app.

    help: Prints this help message.
"#
    )
}
