use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use ring::signature::{self, KeyPair, Signer, VerificationResult};
use ring::rand::SecureRandom;
use ring::error::Unspecified;
use std::sync::Mutex;
use rocket::http::Status;
use rocket::response::status;
use rocket::response;
use rocket::Request;
use rocket::Outcome;
use rocket::Route;
use rocket::Rocket;
use rocket::Config;
use rocket::fs::FileServer;

// 定义一个结构体来存储私钥
#[derive(Debug, Clone)]
struct SignerService {
    private_key: KeyPair,
}

impl SignerService {
    // 创建一个新的签名服务实例
    fn new() -> Result<Self, Unspecified> {
        let rng = SystemRandom::new();
        let private_key = KeyPair::generate(&signature::RSA_PKCS1_2048_8192_SHA256, &rng)?;
        Ok(SignerService { private_key })
    }

    // 对给定的消息进行签名
    fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, Unspecified> {
        let signer = signature::Signer::new(&signature::RSA_PKCS1_SHA256, &self.private_key);
        signer.sign(message)
    }

    // 验证签名
    fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<bool, Unspecified> {
        let verifier = signature::Verifier::new(&signature::RSA_PKCS1_SHA256, &self.private_key.public_key());
        verifier.verify(message, signature).map(|result| result == VerificationResult::Verified)
    }
}

// 定义一个签名请求结构体
#[derive(Deserialize)]
pub struct SignatureRequest {
    message: String,
    signature: Option<String>,
}

// 定义一个签名响应结构体
#[derive(Serialize)]
pub struct SignatureResponse {
    message: String,
    signed: bool,
    error: Option<String>,
}

#[launch]
fn rocket() -> Rocket {
    let signer_service = SignerService::new().expect("Failed to initialize signer service");
    rocket::build()
        .manage(signer_service)
        .mount("/", routes![sign_message, verify_signature])
        .attach(FileServer::from("static"))
}

#[get("/sign")]
fn sign_message(
    signer_service: &State<Mutex<SignerService>>,
    request: &Request<'_>,
) -> Result<status::Custom<Json<SignatureResponse>>, status::Custom<Json<SignatureResponse>>> {
    let message = request.get_query_value("message").unwrap_or_default().as_str().unwrap_or_default();
    match signer_service.lock().unwrap().sign_message(message.as_bytes()) {
        Ok(signature) => Ok(status::Custom(Status::Ok,
            Json(SignatureResponse {
                message,
                signed: true,
                error: None,
            })
        )),
        Err(_) => Ok(status::Custom(Status::InternalServerError,
            Json(SignatureResponse {
                message,
                signed: false,
                error: Some("Failed to sign the message".to_string()),
            })
        )),
    }
}

#[post("/verify")]
fn verify_signature(
    signer_service: &State<Mutex<SignerService>>,
    request: Json<SignatureRequest>,
) -> Result<status::Custom<Json<SignatureResponse>>, status::Custom<Json<SignatureResponse>>> {
    let message = request.message.as_bytes();
    let signature = request.signature.map(|s| s.into_bytes());
    match signature {
        Some(sig) => match signer_service.lock().unwrap().verify_signature(message, &sig) {
            Ok(result) => Ok(status::Custom(Status::Ok,
                Json(SignatureResponse {
                    message: request.message,
                    signed: result,
                    error: None,
                })
            )),
            Err(_) => Ok(status::Custom(Status::InternalServerError,
                Json(SignatureResponse {
                    message: request.message,
                    signed: false,
                    error: Some("Failed to verify the signature".to_string()),
                })
            )),
        None => Ok(status::Custom(Status::BadRequest,
            Json(SignatureResponse {
                message: request.message,
                signed: false,
                error: Some("Signature is required".to_string()),
            })
        )),
    }
}

// 随机数生成器
struct SystemRandom;

impl SecureRandom for SystemRandom {
    fn next_bytes(&self, buffer: &mut [u8]) {
        ring::rand::SystemRandom::new().fill(buffer).unwrap();
    }
}
