use crate::models::{CreateTaskRequest, Task};
use actix_web::{web, HttpResponse, Responder};
use anyhow::{Context, Result};
use reqwest::{Client, Method, Response};
use serde_json::Value;
use std::collections::HashMap;
pub async fn hello() -> impl Responder {
  let params = HttpRequestParams {
    url: "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid=APPID&secret=APPSECRET".to_string(),
    method: Method::GET,
    headers: Some({
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "MyRustClient/1.0".to_string());
        headers
    }),
    body: None,
};

  // 发送请求
  let result = send_request(params).await;
  match result {
      Ok(res) => println!("Response JSON: {}", res),
      Err(err) => println!("Error: {}", err),
  }
  HttpResponse::Ok().body("Hello world!")
}

pub async fn create_task(
  task_data: web::Json<CreateTaskRequest>,
) -> impl Responder {
  println!("{:?}", task_data);
  let task = Task::new(task_data.name.clone());
  HttpResponse::Ok()
    .body("Hello create_task!".to_owned() + task.get_id().to_string().as_str())
}

pub async fn invoke(task_data: web::Json<CreateTaskRequest>) -> impl Responder {
  println!("{:?}", task_data);
  let task = Task::new(task_data.name.clone());
  HttpResponse::Ok()
    .body("Hello create_task!".to_owned() + task.get_id().to_string().as_str())
}

/// HTTP 请求参数结构体
#[derive(Debug)]
pub struct HttpRequestParams {
  pub url: String,
  pub method: Method,
  pub headers: Option<HashMap<String, String>>,
  pub body: Option<String>,
}

/// 发起 HTTP 请求并返回 JSON 结果
pub async fn send_request(params: HttpRequestParams) -> Result<Value> {
  // 创建 HTTP 客户端
  let client = Client::new();

  // 构建请求
  let mut request_builder = client.request(params.method, &params.url);
  // .headers(convert_headers(params.headers));

  // 添加请求体（如果存在）
  if let Some(body) = params.body {
    request_builder = request_builder.body(body);
  }

  // 发送请求
  let response = request_builder
    .send()
    .await
    .context("Failed to send request")?;

  // 处理响应
  handle_response(response).await
}

/// 将 HashMap 转换为 HeaderMap
// fn convert_headers(headers: Option<HashMap<String, String>>) -> HeaderMap {
//     let mut header_map = HeaderMap::new();
//     if let Some(headers) = headers {
//         for (key, value) in headers {
//             if let Ok(header_name) = key.parse() {
//                 if let Ok(header_value) = value.parse() {
//                     header_map.insert(header_name, header_value);
//                 }
//             }
//         }
//     }
//     header_map
// }

/// 处理 HTTP 响应
async fn handle_response(response: Response) -> Result<Value> {
  // 检查 HTTP 状态码
  let status = response.status();
  if !status.is_success() {
    let text = response
      .text()
      .await
      .context("Failed to read error response")?;
    anyhow::bail!("HTTP Error {}: {}", status, text);
  }

  // 解析 JSON 响应
  let json = response
    .json::<Value>()
    .await
    .context("Failed to parse JSON response")?;

  Ok(json)
}
