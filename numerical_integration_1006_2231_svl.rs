use rocket::get;
use rocket::serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::status;
use std::f64;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 定义一个请求结构体用于接收积分参数
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegralParams {
    fn_name: String,
    a: f64,
    b: f64,
    num: u32,
}

// 定义一个响应结构体用于返回积分结果
#[derive(Serialize, Deserialize, Debug)]
pub struct IntegralResult {
    result: f64,
}

// 定义一个服务，用于处理积分计算
pub struct NumericalIntegrationService;

impl NumericalIntegrationService {
    // 定义一个方法计算定积分
    pub fn compute_integral(params: IntegralParams) -> Result<IntegralResult, String> {
        match params.fn_name.as_str() {
            "sin" => Self::sin_integral(&params),
            "cos" => Self::cos_integral(&params),
            _ => Err("Function not supported".to_string()),
        }
    }

    // 计算 sin(x) 在区间 [a, b] 上的积分
    fn sin_integral(params: &IntegralParams) -> Result<IntegralResult, String> {
        let h = (params.b - params.a) / params.num as f64;
        let mut sum = 0.0;
        for i in 0..params.num {
            let x = params.a + i as f64 * h;
            sum += (i * h).sin();
        }
        Ok(IntegralResult { result: sum * 2.0 / params.num as f64 })
    }

    // 计算 cos(x) 在区间 [a, b] 上的积分
    fn cos_integral(params: &IntegralParams) -> Result<IntegralResult, String> {
        let h = (params.b - params.a) / params.num as f64;
        let mut sum = 0.0;
        for i in 0..params.num {
            let x = params.a + i as f64 * h;
            sum += (i * h).cos();
        }
        Ok(IntegralResult { result: sum * 2.0 / params.num as f64 })
    }
}

// 定义一个火箭路由处理程序
#[get("/integral?<params>")]
fn compute(params: IntegralParams) -> Result<status::Custom<IntegralResult>, status::Custom<String>> {
    match NumericalIntegrationService::compute_integral(params) {
        Ok(result) => Ok(status::Custom(
            result,
            Status::Ok,
        )),
        Err(e) => Err(status::Custom(e, Status::BadRequest)),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![compute])
}