// integration_test.rs
// 这是一个使用RUST和ROCKET框架的集成测试工具。
// 它提供了一个基本的测试用例，用于验证ROCKET应用的基本功能。

use rocket::http::Status;
use rocket::local::blocking::Client;
use rocket::serde::json::Json;
use serde::Deserialize;
use super::rocket; // 引入rocket模块，假设rocket模块中包含了我们的rocket应用

#[derive(Deserialize)]
pub struct TestResponse {
    message: String,
}

#[rocket::async_test]
async fn test_get() {
    // 创建一个ROCKET客户端，用于发送请求
    let client = Client::new(rocket()).expect("valid rocket instance");

    // 发送GET请求到'/'路径
    let response = client.get("/").dispatch().await;

    // 检查响应状态码是否为200 OK
    assert_eq!(response.status(), Status::Ok);

    // 将响应体解析为JSON，并检查其内容
    let body = response.into_json::<TestResponse>().await;
    assert!(body.is_ok());
    let body = body.unwrap();
    assert_eq!(body.message, "Hello, world!");
}
