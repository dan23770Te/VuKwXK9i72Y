use rocket::get;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket::State;
use sysinfo::{System, SystemExt};

// 定义系统监控数据结构
#[derive(serde::Serialize, serde::Deserialize)]
struct SystemPerformance {
    total_memory: u64,
    used_memory: u64,
    free_memory: u64,
    total_swap: u64,
    used_swap: u64,
    free_swap: u64,
    cpu_usage: f32,
    process_count: u64,
    current_load: f32,
}

// 系统监控服务
#[rocket::get("/system-performance")]
fn system_performance(sys: &State<System>) -> Json<SystemPerformance> {
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let free_memory = sys.free_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let free_swap = sys.free_swap();
    let cpu_usage = sys.cpu_usage();
    let process_count = sys.processes().len() as u64;
    let current_load = sys.load_average().get_load_average();

    Json(SystemPerformance {
        total_memory,
        used_memory,
        free_memory,
        total_swap,
        used_swap,
        free_swap,
        cpu_usage,
        process_count,
        current_load,
    })
}

#[launch]
fn rocket() -> _ {
    let mut config = rocket::Config::debug_default();
    config
        .mount("/api", rocket::routes![system_performance])
        .manage(System::new_all());

    rocket::custom(config)
}

// 系统监控工具的主入口
fn main() {
    // 启动ROCKET服务器
    rocket().launch();
}
