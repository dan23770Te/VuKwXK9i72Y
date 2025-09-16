use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Value;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use flate2::read::GzDecoder;
use flate2::read::ZipFile;
use zip::ZipArchive;
use rocket::response::status;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
#[serde(crate = "serde")]
struct DecompressRequest {
    file_path: String,
    output_path: String
}

#[get("/decompress")]
fn decompress(request: Json<DecompressRequest>) -> Result<status::Custom<Json<Value>>, status::Custom<Json<Value>>> {
    // Extract file paths from the request
    let file_path = &request.file_path;
    let output_path = &request.output_path;
    let path = Path::new(file_path);
    let output_dir = Path::new(output_path);

    // Check if file exists
    if !path.exists() {
        return Err(status::Custom(Status::BadRequest, Json(json!({
            "error": "File does not exist"
        }))))
    }

    // Decompress file
    match decompress_file(path, output_dir) {
        Ok(_) => Ok(status::Custom(Status::Ok, Json(json!({
            "message": "File decompressed successfully"
        })))),
        Err(e) => Err(status::Custom(Status::InternalServerError, Json(json!({
            "error": e.to_string()
        })))),
    }
}

fn decompress_file<P: AsRef<Path>, Q: AsRef<Path>>(file_path: P, output_dir: Q) -> Result<(), std::io::Error> {
    let file = fs::File::open(file_path)?;
    let path = file_path.as_ref();

    // Check file extension to determine decompression method
    if path.extension().and_then(|s| s.to_str()) == Some("gz") {
        decompress_gz(file, output_dir)?;
    } else if path.extension().and_then(|s| s.to_str()) == Some("zip") {
        decompress_zip(file, output_dir)?;
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported file format"));
    }

    Ok(())
}

fn decompress_gz<R: Read + Seek>(reader: R, output_dir: &Path) -> Result<(), std::io::Error> {
    let mut gz = GzDecoder::new(reader);
    let mut buf_reader = BufReader::new(gz);
    let mut content = Vec::new();
    buf_reader.read_to_end(&mut content)?;
    fs::write(output_dir.join("output.txt"), content)?;
    Ok(())
}

fn decompress_zip<R: Read + Seek>(reader: R, output_dir: &Path) -> Result<(), std::io::Error> {
    let mut archive = ZipArchive::new(reader)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = output_dir.join(file.name());
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/decompress", routes![decompress])
}