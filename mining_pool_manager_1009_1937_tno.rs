 * along with error handling and follows Rust best practices for maintainability and scalability.
 */

use rocket::get;
use rocket::post;
use rocket::put;
use rocket::delete;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::serde::json::Json as RocketJson;
use rocket::State;
use std::collections::HashMap;

// Define the structure of a mining pool
#[derive(Debug, Serialize, Deserialize, Clone)]
struct MiningPool {
    id: u32,
    name: String,
    capacity: u32,
    miners: Vec<String>,
}

// Define the MiningPoolManager to handle the pools
struct MiningPoolManager {
    pools: HashMap<u32, MiningPool>,
    next_id: u32,
}

// Implementation of MiningPoolManager
impl MiningPoolManager {
    fn new() -> Self {
        MiningPoolManager {
            pools: HashMap::new(),
            next_id: 1,
        }
    }

    // Function to add a new mining pool
    fn add_pool(&mut self, name: String, capacity: u32) -> &MiningPool {
        let pool = MiningPool {
            id: self.next_id,
            name,
            capacity,
            miners: Vec::new(),
        };
        self.pools.insert(self.next_id, pool);
        self.next_id += 1;
        &self.pools[&self.next_id - 1]
    }

    // Function to remove a mining pool
    fn remove_pool(&mut self, id: u32) -> Option<MiningPool> {
        self.pools.remove(&id)
    }

    // Function to get a mining pool by id
    fn get_pool(&self, id: u32) -> Option<&MiningPool> {
        self.pools.get(&id)
    }
}

// Define routes for the mining pool management API
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut pool_manager = MiningPoolManager::new();
    rocket::build()
        .manage(pool_manager)
        .mount("/", routes![
            create_pool,
            delete_pool,
            get_pool,
            update_pool,
            list_pools,
        ])
        .launch()
        .await?
}

// Route to create a new mining pool
#[post("/pool", format = "json", data = "<pool>")]
fn create_pool(pool: Json<MiningPool>, pool_manager: &State<MiningPoolManager>) -> Created<RocketJson<MiningPool>> {
    let new_pool = pool_manager.add_pool(pool.name.clone(), pool.capacity);
    Created::new("/pool/".to_string() + &new_pool.id.to_string())
        .body(new_pool.clone())
}

// Route to delete a mining pool
#[delete("/pool/<id>")]
fn delete_pool(id: u32, pool_manager: &State<MiningPoolManager>) -> Option<MiningPool> {
    pool_manager.remove_pool(id)
}

// Route to get a mining pool by id
#[get("/pool/<id>")]
fn get_pool(id: u32, pool_manager: &State<MiningPoolManager>) -> Option<Json<MiningPool>> {
    pool_manager.get_pool(id).map(Json)
}

// Route to update a mining pool
#[put("/pool/<id>", format = "json", data = "<pool>")]
fn update_pool(id: u32, pool: Json<MiningPool>, pool_manager: &State<MiningPoolManager>) -> Option<Json<MiningPool>> {
    pool_manager.get_pool(id).map(|mut p| {
        p.name = pool.name;
        p.capacity = pool.capacity;
        p.miners = pool.miners;
        p
    }).map(Json)
}

// Route to list all mining pools
#[get("/pools")]
fn list_pools(pool_manager: &State<MiningPoolManager>) -> Json<Vec<MiningPool>> {
    Json(pool_manager.pools.values().cloned().collect())
}
