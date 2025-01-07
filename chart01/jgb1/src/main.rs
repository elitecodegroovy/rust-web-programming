use std::collections::HashMap;
use reqwest::Error;

async fn get_request() -> Result<(), Error>{
    let resp = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json::<HashMap<String,  String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}

async fn post_request() -> Result<(), Error> {
    let url = "https://ip.taobao.com/outGetIpInfo?ip=183.6.57.51&accessKey=alibaba-inc";
    let response = reqwest::Client::new()
        .post(url)
        .send()
        .await?;
    if response.status() == 200 {
        let response_body = response.text().await?;
        println!("请求返回数据: \n{}", response_body);
    }

    Ok(())

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    get_request().await?;
    post_request().await?;
    Ok(())
}