use rocket::get;
use rocket::Route;
use rocket::serde::json::Json;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use rocket::response::status;
use rocket::serde::json::serde_json::json;

// 定义日志解析器结构体
struct LogParser {
    file_path: String,
}

// 定义日志记录结构体
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct LogRecord {
    timestamp: String,
    level: String,
    message: String,
}

#[get("/parse_log/<file_path>")]
fn parse_log(file_path: String) -> status::Custom<Json<Vec<LogRecord>>> {
    let path = Path::new(&file_path);
    if !path.exists() || !path.is_file() {
        return status::Custom(
            status::NotFound,
            Json(json!({
                "error": "File does not exist",
            })),
        );
    }

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return status::Custom(
            status::BadRequest,
            Json(json!({
                "error": format!("Failed to open file: {}", e),
            })),
        ),
    };

    let mut records = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // 简单的示例解析器，假设日志格式为："[timestamp] [level] message"
                let parts: Vec<&str> = line.splitn(3, ' ').collect();
                if parts.len() == 3 {
                    let record = LogRecord {
                        timestamp: parts[0].to_string(),
                        level: parts[1].to_string(),
                        message: parts[2].to_string(),
                    };
                    records.push(record);
                }
            },
            Err(e) => return status::Custom(
                status::InternalServerError,
                Json(json!({
                    "error": format!("Failed to read line: {}", e),
                })),
            ),
        }
    }

    status::Custom(
        status::Ok,
        Json(records),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![parse_log])
}