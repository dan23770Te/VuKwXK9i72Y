use rocket::form::Form;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status;
use rocket::State;
use image::{DynamicImage, GenericImageView, ImageError};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use serde_json::json;
use std::{fs, path::PathBuf};

// 定义图像尺寸调整请求的结构体
#[derive(Deserialize)]
pub struct ResizeRequest {
    width: u32,
    height: u32,
    files: Vec<PathBuf>,
}

// 定义响应结构体
#[derive(Serialize)]
pub struct ResizeResponse {
    success: bool,
    message: String,
}

#[rocket::get("/")]
fn index() -> &'static str {
    "Image Resizer Service"
}

#[post("/resize", format = "json", data = "<request>")]
async fn resize_images(
    request: Json<ResizeRequest>,
    rocket: &State<Rocket>,
) -> status::Accepted<Json<ResizeResponse>> {
    let resize_response = resize_images_helper(&request.into_inner(), rocket);
    match resize_response {
        Ok(_) => Ok(Json(ResizeResponse {
            success: true,
            message: "Images resized successfully.".to_string(),
        })),
        Err(e) => Ok(Json(ResizeResponse {
            success: false,
            message: e.to_string(),
        })),
    }
}

fn resize_images_helper(request: &ResizeRequest, rocket: &Rocket) -> Result<(), ImageError> {
    for file_path in &request.files {
        let img = image::open(file_path).map_err(|e| {
            println!("Error opening image file {}: {}", file_path.display(), e);
            e
        })?;
        let resized_img = img.resize(request.width, request.height, image::imageops::FilterType::Nearest);
        let mut output_file = File::create(&format!("{}_resized.{}", file_path.display(), img.format().to_string().to_lowercase()))?;
        resized_img.write_to(&mut output_file, img.format())?;
    }
    Ok(())
}

// 定义Rocket配置结构体
#[derive(Deserialize, Serialize)]
struct Rocket {
    public_address: String,
    public_port: u16,
}

// 定义Rocket配置
fn rocket_config() -> Rocket {
    Rocket {
        public_address: "0.0.0.0".to_string(),
        public_port: 8000,
    }
}

fn main() {
    let rocket_config = rocket_config();
    rocket::build()
        .manage(rocket_config)
        .mount("/", routes![index, resize_images])
        .launch()
        .expect("Server launch failed");
}
