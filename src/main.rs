#![allow(dead_code)]
use reqwest::{
    cookie::CookieStore,
    header::{CONTENT_LENGTH, COOKIE},
};
use serde::Deserialize;
use std::collections::HashMap;

//fn main() {
//    blocking_get().unwrap();
//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tokio::task::spawn_blocking(move || blocking_get().unwrap());
    //basic().await?;
    //json().await?;
    //post().await?;
    //status().await?;
    //request().await?;
    //cookies().await?;
    //proxy().await?;

    Ok(())
}

fn blocking_get() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get("http://0.0.0.0:8000")?;

    let body = res.text()?;
    println!("body = {:?}", body);

    Ok(())
}

async fn basic() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("http://0.0.0.0:8000").await?.text().await?;
    println!("body = {:?}", body);

    Ok(())
}

#[derive(Deserialize)]
struct Data {
    test: String,
}

async fn json() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("http://0.0.0.0:8000").await?;

    let body = res.json::<Data>().await?;
    println!("body = {:?}", body.test);

    Ok(())
}

async fn post() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://0.0.0.0:8000")
        .header(CONTENT_LENGTH, 27)
        .timeout(tokio::time::Duration::from_secs(5))
        .body("the exact body that is sent")
        .send()
        .await?;

    let body = res.text().await?;
    println!("body = {:?}", body);

    Ok(())
}

async fn post_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("id", "33");
    map.insert("payload", "some data here");

    let client = reqwest::Client::new();
    client.post("http://0.0.0.0:8000").json(&map).send().await?;

    Ok(())
}

async fn status() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post("http://0.0.0.0:8000").send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            println!("StatusCode is OK");
        }
        status => {
            println!("StatusCode is {:?}", status);
        }
    }

    Ok(())
}

async fn request() -> Result<(), Box<dyn std::error::Error>> {
    let mut req = reqwest::Request::new(
        reqwest::Method::GET,
        reqwest::Url::parse("http://0.0.0.0:8001")?,
    );

    let url = req.url_mut();
    println!("PORT: {}", url.port().unwrap());
    url.set_port(Some(8000)).unwrap();
    println!("PORT: {}", url.port().unwrap());

    let client = reqwest::Client::new();
    let res = client.execute(req).await?;

    let body = res.text().await?;
    println!("body = {:?}", body);

    Ok(())
}

async fn cookies() -> Result<(), Box<dyn std::error::Error>> {
    let jar = reqwest::cookie::Jar::default();
    let cookie_store = std::sync::Arc::new(jar);

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(cookie_store.clone())
        .build()?;

    client
        .get("http://0.0.0.0:8000")
        .header(COOKIE, "test")
        .send()
        .await?;

    Ok(())
}

async fn proxy() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("socks5://127.0.0.1:9050")?)
        .build()?;

    let res = client.get("https://check.torproject.org/").send().await?;

    let body = res.text().await?;
    println!("body = {:?}", body);

    Ok(())
}
