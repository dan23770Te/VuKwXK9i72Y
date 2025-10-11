use rocket::Route;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

// Define a structure to represent test results.
#[derive(Serialize, Deserialize, Debug)]
pub struct TestResult {
    test_id: String,
    test_name: String,
    test_description: String,
    passed: bool,
    duration: f64, // Duration in seconds
    result_details: HashMap<String, String>, // Additional details about the test result
}

// Define a service to handle test result analysis.
pub struct TestResultAnalyser;

impl TestResultAnalyser {
    // Analyze test results and return a summary.
    pub fn analyze_results(results: Vec<TestResult>) -> Result<String, Box<dyn Error>> {
        let mut summary = String::new();
        
        for result in results {
            if result.passed {
                summary.push_str(&format!("Test '{}' passed in {:.2f} seconds.
", result.test_name, result.duration));
            } else {
                summary.push_str(&format!("Test '{}' failed in {:.2f} seconds.
", result.test_name, result.duration));
            }
        }
        
        Ok(summary)
    }
}

// Define routes for the Rocket server.
#[macro_export]
macro_rules! routes {
    () => {
        vec![
            Route::get("/", ranks),
            Route::post("/results", analyze_results),
        ]
    }
}

// Define a handler function to analyze test results.
#[post("/results", format = "json", data = "<results>")]
fn analyze_results(results: Vec<TestResult>) -> String {
    match TestResultAnalyser::analyze_results(results) {
        Ok(summary) => summary,
        Err(e) => e.to_string(),
    }
}

// Define the Rocket configuration.
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes!())
}


// Main function to run the application.
fn main() {
    if let Err(e) = rocket().launch() {
        eprintln!("Failed to start the server: {}", e);
    }
}
