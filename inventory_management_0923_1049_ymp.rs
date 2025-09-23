use rocket::get;
use rocket::post;
# 扩展功能模块
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::response::status;
use std::sync::Mutex;
# 改进用户体验
use std::sync::Arc;
use std::collections::HashMap;

// 定义库存项
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
# TODO: 优化性能
    name: String,
    quantity: u32,
# NOTE: 重要实现细节
}

// 定义库存管理系统
# 改进用户体验
struct InventoryManager {
    items: Mutex<HashMap<u32, Item>>,
}

#[post("/add_item", format = "json", data = "<item>")]
fn add_item(item: Json<Item>, manager: rocket::State<Arc<InventoryManager>>) -> status::StatusCode {
    let mut items = manager.items.lock().unwrap();
    items.insert(item.id, item.into_inner());
    status::StatusCode::CREATED
}

#[get("/items")]
fn get_items(manager: rocket::State<Arc<InventoryManager>>) -> Json<Vec<Item>> {
# NOTE: 重要实现细节
    let items = manager.items.lock().unwrap();
# 改进用户体验
    let mut items_vec: Vec<Item> = items.values().cloned().collect();
    items_vec.sort_by_key(|item| item.id);
    Json(items_vec)
}

#[get("/item/<id>")]
# 添加错误处理
fn get_item(id: u32, manager: rocket::State<Arc<InventoryManager>>) -> Result<Json<Item>, status::StatusCode> {
    let items = manager.items.lock().unwrap();
    match items.get(&id) {
        Some(item) => Ok(Json(item.clone())),
        None => Err(status::StatusCode::NOT_FOUND),
# 添加错误处理
    }
}
# TODO: 优化性能

#[post("/update_item/<id>", format = "json", data = "<item>")]
fn update_item(id: u32, item: Json<Item>, manager: rocket::State<Arc<InventoryManager>>) -> Result<status::StatusCode, status::StatusCode> {
    let mut items = manager.items.lock().unwrap();
    match items.get_mut(&id) {
# TODO: 优化性能
        Some(existing_item) => {
            existing_item.name = item.name.clone();
            existing_item.quantity = item.quantity;
            Ok(status::StatusCode::OK)
        },
        None => Err(status::StatusCode::NOT_FOUND),
    }
}

#[post("/delete_item/<id>")]
fn delete_item(id: u32, manager: rocket::State<Arc<InventoryManager>>) -> Result<status::StatusCode, status::StatusCode> {
    let mut items = manager.items.lock().unwrap();
    match items.remove(&id) {
        Some(_) => Ok(status::StatusCode::NO_CONTENT),
        None => Err(status::StatusCode::NOT_FOUND),
    }
}

#[launch]
fn rocket() -> _ {
# FIXME: 处理边界情况
    rocket::build()
        .manage(Arc::new(InventoryManager {
            items: Mutex::new(HashMap::new()),
        }))