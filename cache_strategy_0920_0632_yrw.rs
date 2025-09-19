use rocket::fairing::Fairing;
use rocket::http::{Header, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Request, Outcome};
use rocket::response::{self, Response, Responder};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};
use rocket::tokio::sync::Mutex as AsyncMutex;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde_json::json;

// 缓存条目的结构体
#[derive(Serialize, Deserialize, Clone, Debug)]
struct CacheEntry {
    data: String,
    expires_at: SystemTime,
}

// 缓存服务的实现
struct CacheService {
    cache: AsyncMutex<HashMap<String, CacheEntry>>,
}

impl<'r> request::FromRequest<'r> for CacheService {
    type Error = ();

    fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cache_service = request.guard::<rocket::fairing::AdHoc>().unwrap()
            .get_one::<CacheService>().unwrap().clone();
        Outcome::Success(cache_service)
    }
}

// 缓存策略的公平性
struct CacheStrategy;

#[rocket::async_trait]
impl Fairing for CacheStrategy {
    fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let cache_service = request.guard::<CacheService>().unwrap();
        let mut cache = cache_service.cache.lock().unwrap();

        if let Some(CacheEntry { data, expires_at }) = cache.get(&request.uri().to_string()) {
            if SystemTime::now() >= *expires_at {
                cache.remove(&request.uri().to_string());
            } else {
                let response_data = response.body_string().or_else(|_| Some(String::from("No body available"))).unwrap();
                if response_data == data {
                    // 如果缓存数据与响应数据一致，则直接返回缓存数据
                    *response.body_mut() = Some(response_data.into());
                }
            }
        }
    }

    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Cache Strategy",
            kind: rocket::fairing::Kind::Response,
       }
    }
}

// 启动缓存服务
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/", routes![cache_data])
        .attach(CacheStrategy)
        .manage(CacheService {
            cache: AsyncMutex::new(HashMap::new()),
        })
}

// 缓存数据的API
#[get("/cache_data")]
async fn cache_data(cache_service: CacheService) -> impl Responder<'static> {
    let mut cache = cache_service.cache.lock().await;
    let cache_key = "/api/cache_data".to_string();
    let cache_entry = cache.entry(cache_key.clone()).or_insert_with(|| {
        CacheEntry {
            data: "Cached data".to_string(),
            expires_at: SystemTime::now() + Duration::from_secs(60),
        }
    });

    if SystemTime::now() >= cache_entry.expires_at {
        cache_entry.data = "Cached data updated".to_string();
        cache_entry.expires_at = SystemTime::now() + Duration::from_secs(60);
    }

    json!({
        "data": cache_entry.data.clone(),
        "expires_at": cache_entry.expires_at,
    })
}
