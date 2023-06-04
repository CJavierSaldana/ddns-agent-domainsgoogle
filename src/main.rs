use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Starting DDNS updater");

    let ddns_user = std::env::var("DDNS_USER").expect("DDNS_USER is not set");
    let ddns_pass = std::env::var("DDNS_PASS").expect("DDNS_PASS is not set");
    let ddns_host = std::env::var("DDNS_HOST").expect("DDNS_HOST is not set");

    println!("ENVs loaded");

    let public_ip = match get_public_ip().await {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Error getting public IP: {}", e);
            return Ok(());
        }
    };

    println!("Public IP: {}", public_ip);

    let ddnsresult = match set_ip_ddns(public_ip, ddns_host, ddns_user, ddns_pass).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error updating DDNS: {}", e);
            return Ok(());
        }
    };

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

    let auth = format!("{}:{}", user, pass);

    headers.insert(
        "Authorization",
        format!("Basic {}", base64::encode(auth)).parse()?,
    );

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
