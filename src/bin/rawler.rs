use rawler::crawl;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("http://0.0.0.0:80")?;
    crawl(url).await
}
