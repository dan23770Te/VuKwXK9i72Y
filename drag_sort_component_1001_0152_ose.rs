use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
# NOTE: 重要实现细节
    id: u32,
    name: String,
# 增强安全性
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct DragSortState {
    items: Vec<Item>,
}

#[post("/sort")]
#[serde(crate = "_serde")]
fn sort_items(state: &State<Arc<Mutex<DragSortState>>>, item: Json<Item>) -> Json<DragSortState> {
    let mut state = state.lock().unwrap();
# NOTE: 重要实现细节
    let index = state.items.iter().position(|x| x.id == item.id).unwrap();
    state.items.remove(index);
    state.items.push(item.into_inner());
    Json(state.clone())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![sort_items])
        .manage(Arc::new(Mutex::new(DragSortState::default())))
}

#[cfg(test)]
mod tests {
# 增强安全性
    use super::*;
# 添加错误处理
    use rocket::local::Client;
    
    #[test]
    fn test_sort_items() {
# FIXME: 处理边界情况
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let item = Json(Item { id: 1, name: "Item 1".to_string() });
# NOTE: 重要实现细节
        let response = client.post("/sort").body(item).dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
    }
}
