use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Starting DDNS updater");

    // get the env variables DDNS_USER
    let ddns_user = std::env::var("DDNS_USER").unwrap_or_default();

    // get the env variables DDNS_PASS
    let ddns_pass = std::env::var("DDNS_PASS").unwrap_or_default();

    // get the env variables DDNS_HOST
    let ddns_host = std::env::var("DDNS_HOST").unwrap_or_default();

    // validate the env variables
    if ddns_user == "" || ddns_pass == "" {
        println!("DDNS_USER or DDNS_PASS is not set");
        return Ok(());
    }

    // validate the env variables
    if ddns_host == "" {
        println!("DDNS_HOST is not set");
        return Ok(());
    }

    // print ok message
    println!("DDNS_USER and DDNS_PASS are set");

    // get public ip and set it to a variable
    let public_ip = get_public_ip().await.unwrap();

    // print the public ip
    println!("Public IP: {}", public_ip);

    // set the ip to the ddns
    let ddnsresult = set_ip_ddns(public_ip, ddns_host, ddns_user, ddns_pass)
        .await
        .unwrap();

    // print the ddns result
    println!("DDNS Result: {}", ddnsresult);

    Ok(())
}

async fn get_public_ip() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let request = client.request(reqwest::Method::GET, "https://api.seeip.org");

    let response = request.send().await?;
    let body = response.text().await?;

    Ok(body)
}

async fn set_ip_ddns(
    ip: String,
    host: String,
    user: String,
    pass: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();

    // calculate the base64 auth
    let auth = format!("{}:{}", user, pass);

    // insert the auth header
    headers.insert(
        "Authorization",
        format!("Basic {}", base64::encode(auth)).parse()?,
    );

    // compose the url
    let url = format!(
        "https://domains.google.com/nic/update?hostname={}&myip={}",
        host, ip
    );

    let request = client
        .request(reqwest::Method::GET, url.as_str())
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    Ok(body)
}
