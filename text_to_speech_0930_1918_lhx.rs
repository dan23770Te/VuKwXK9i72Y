// 使用Rust标准库
use std::sync::Arc;

// 使用Rocket框架
#[macro_use]
extern crate rocket;

// 使用gTTS库进行文本到语音的转换
extern crate gtts;

// 引入Rocket和gTTS的相关模块
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use gtts::gTTS;

// 定义一个全局状态，存储语音合成器的配置
#[derive(Debug, Clone)]
struct VoiceConfig {
    language: String,
    slow: bool,
};

// 实现从JSON解析配置的功能
#[derive(Deserialize)]
struct VoiceRequest {
    text: String,
    config: VoiceConfig,
};

// 定义一个处理文本到语音请求的服务
#[get("/synthesize")]
#[catch(default)]
fn synthesize语音<'r>(req: &'r VoiceRequest,
                      config: &State<Arc<VoiceConfig>>) -> status::Custom<'r> {
    // 检查请求中的文本是否为空
    if req.text.is_empty() {
        return status::Custom(Status::BadRequest, "Text cannot be empty");
    }

    // 创建gTTS对象
    let mut tts = gTTS::new(&req.text);
    tts
        .set_lang(&config.language)
        .set_slow(req.config.slow);

    // 尝试生成MP3文件
    match tts.save("./output.mp3") {
        Ok(_) => status::Custom(Status::Ok, "Audio file generated successfully"),
        Err(e) => status::Custom(Status::InternalServerError, format!("Failed to generate audio: {}", e)),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![synthesize语音])
        .manage(Arc::new(VoiceConfig {
            language: "en".to_string(),
            slow: false,
        }));
}

// 定义Rocket的路由
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_text() {
        let req = VoiceRequest {
            text: "".to_string(),
            config: VoiceConfig {
                language: "en".to_string(),
                slow: false,
            },
        };
        let result = synthesize语音(&req, &State::from(Arc::new(VoiceConfig {
            language: "en".to_string(),
            slow: false,
        })));
        assert_eq!(result.status(), Status::BadRequest);
    }
}
