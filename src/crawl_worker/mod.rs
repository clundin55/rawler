use crate::{CrawlDepth, CrawlError};
use regex::Regex;
use std::fmt;
use std::sync::mpsc::Sender;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrawledItem {
    url: Url,
    depth: CrawlDepth,
}

impl CrawledItem {
    pub fn new(url: Url, depth: u32) -> Self {
        CrawledItem { url, depth }
    }
    fn get_url(&self) -> &Url {
        &self.url
    }
    pub fn get_depth(&self) -> &CrawlDepth {
        &self.depth
    }
}

impl fmt::Display for CrawledItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

pub async fn crawl_job(
    crawled_url: CrawledItem,
    crawl_queue: Sender<CrawledItem>,
) -> Result<Vec<CrawledItem>, CrawlError> {
    // Safe to return an error, here as the queue should not be empty
    let mut children = Vec::new();
    if let Ok(body) = reqwest::get(crawled_url.get_url().clone()).await {
        for cap in find_urls(&body.text().await?)? {
            if let Ok(new_url) = Url::parse(&cap) {
                let nested_page = CrawledItem::new(new_url.clone(), crawled_url.get_depth() + 1);
                crawl_queue.send(nested_page.clone())?;
                children.push(nested_page);
            }
        }
    }

    Ok(children)
}

/// Sample capture:
/// <a href="https://kennethreitz.org" target="_blank"> => https://github.com/requests/httpbin
/// For now, instead of capturing bad addresses in the Regex, we will just let the request fail.
fn find_urls(html_body: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Safe to unwrap here since the string is compiled into the binary, a runtime panic would be
    // very unexpected.
    // Use a non-greedy match so we capture what is the href value.
    let re = Regex::new("href=\"(.*?)\"").unwrap();
    let matches = re.captures_iter(&html_body);
    Ok(matches
        .map(|matched_url| format!("{}", matched_url[1].to_string()))
        .collect())
}

#[cfg(test)]
mod find_urls_tests {
    use super::find_urls;
    #[test]
    fn single_url() {
        let input = "<a href=\"https://example.com\" target=\"_blank\">";
        let expected_output = vec!["https://example.com".to_string()];
        assert_eq!(expected_output, find_urls(input).unwrap());
    }
    #[test]
    fn multiple_urls() {
        let input = "\
            <a href=\"https://example.com\">\
            <a href=\"https://example2.com\" target=\"_blank\">\
            <a href=\"https://helloworld.com\">";
        let expected_output = vec![
            "https://example.com".to_string(),
            "https://example2.com".to_string(),
            "https://helloworld.com".to_string(),
        ];
        assert_eq!(expected_output, find_urls(input).unwrap());
    }
}
