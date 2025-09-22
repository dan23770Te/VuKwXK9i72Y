use rocket::get;
use rocket::Route;
use sysinfo::{System, SystemExt};

/// Fetches and returns system memory usage statistics.
#[get("/memory")]
fn memory_usage() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let memory_usage = sys.get_memory();
    let total_memory = memory_usage.total as f64;
    let used_memory = memory_usage.used as f64;
    let free_memory = memory_usage.free as f64;

    format!(
        "Total Memory: {:.2} MB
Used Memory: {:.2} MB
Free Memory: {:.2} MB",
        total_memory / 1024.0 / 1024.0,
        used_memory / 1024.0 / 1024.0,
        free_memory / 1024.0 / 1024.0
    )
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::build().mount("/api", routes![memory_usage])
}

/// Main function to start the Rocket server.
fn main() {
    // Start the Rocket server.
    rocket().launch();
}
