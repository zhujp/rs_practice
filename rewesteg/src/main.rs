use reqwest::{Client,Error};
use serde::{Deserialize,Serialize};
use serde_json::json;


#[derive(Debug,Deserialize,Serialize)]
struct User {
    name:String,
    pwd:String,
}

#[tokio::main]
async fn main() -> Result<(),Error> {
    //get 请求
    //reqwest 使用了 async/await 语法，因此你需要一个异步运行时，比如 tokio
    // let url = String::from("https://www.baidu.com");
    // let resp = req_get(&url).await?;
    // println!("{}",resp);
    Ok(())
}

async fn req_get(url:&str)-> Result<String,Error>{
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}


async fn req_post(url:&str,post_data:&User)->Result<(),Error> {
    let client = Client::new();
    let resp = client.post(url)
        .json(&post_data)
        .send()
        .await?;
    println!("Status: {}", resp.status()); // 打印状态码。
    println!("Headers:\n{:#?}", resp.headers()); // 打印响应头。
    let json_resp: User = resp.json().await?; // 将响应体解析为 JSON 并反序列化为 PostData 结构体。
    println!("Response Body: {:?}", json_resp); 
    Ok(())
}

