use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Error;
use chrono;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting...");

    loop {
        let is_auth = check_auth().await.unwrap();
        if !is_auth {
            login().await.unwrap();
        }
        thread::sleep(Duration::from_secs(1));
    }
}

async fn check_auth() -> Result<bool, Error> {
    let response = reqwest::get("http://free.wi-mesh.vn/status")
        .await?
        .text()
        .await?;

    if response.contains("KẾT NỐI THÀNH CÔNG!") {
        Ok(true)
    } else {
        Ok(false)
    }
}

async fn login() -> Result<bool, Error> {
    let mut form = HashMap::new();
    form.insert("dst", "");
    form.insert("dst2", "");
    form.insert("popup", "true");
    form.insert("username", "awing60");
    form.insert("password", "Awing60@2018");

    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate"));
    headers.insert("Accept-Language", HeaderValue::from_static("en-VN,en;q=0.9"));
    headers.insert("Cache-Control", HeaderValue::from_static("max-age=0"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert("Cookie", HeaderValue::from_str(&format!("exRun=EX_LOGIN;")).unwrap());
    headers.insert("Origin", HeaderValue::from_static("http://free.wi-mesh.vn"));
    headers.insert("Referer", HeaderValue::from_static("http://free.wi-mesh.vn/login"));
    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"));

    let client = reqwest::Client::new();

    let response = client
        .post("http://free.wi-mesh.vn/login")
        .headers(headers)
        .form(&form)
        .send()
        .await?
        .text()
        .await?;

    if response.contains("Bạn Đã Đăng Nhập Thành Công") {
        println!("Logged in at {}", chrono::Local::now().to_rfc2822());
        Ok(true)
    } else {
        println!("Login failed.");
        Ok(false)
    }
}

async fn _logout() -> Result<bool, Error> {
    println!("Logging out...");
    reqwest::get("http://free.wi-mesh.vn/logout")
        .await?;

    Ok(true)
}
