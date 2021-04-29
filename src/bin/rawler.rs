use rawler::{crawl, CrawlDepth, CrawlError};
use std::time::Duration;
use url::Url;

use clap::{value_t, App, Arg};

#[tokio::main]
async fn main() -> Result<(), CrawlError> {
    let matches = App::new("Rawler")
        .version("1.0")
        .author("Carl Lundin <carllundin55@gmail.com>")
        .about("A URL Crawler.")
        .arg(
            Arg::with_name("depth")
                .short("d")
                .long("depth")
                .value_name("DEPTH")
                .help("How many nested paged to crawl.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("The URL to start the web crawling.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .value_name("TIMEOUT")
                .help("The amount of seconds for the crawler to wait on a page.")
                .takes_value(true),
        )
        .get_matches();

    let depth = value_t!(matches, "depth", CrawlDepth).unwrap_or(2);
    let timeout = value_t!(matches, "timeout", u64).unwrap_or(2);
    let url = value_t!(matches, "url", String).expect("Need a valid URL to start the crawl job.");

    let url = Url::parse(&url)?;
    crawl(url, depth, Duration::from_secs(timeout)).await
}
