use regex::Regex;
use std::collections::{HashSet, VecDeque};
use url::Url;

pub async fn crawl(root_url: Url) -> Result<(), Box<dyn std::error::Error>> {
    let mut url_queue = VecDeque::new();
    let mut seen_urls: HashSet<Url> = HashSet::new();

    url_queue.push_back(root_url.clone());
    seen_urls.insert(root_url.clone());

    // Safe to unwrap here since the string is compiled into the binary, a runtime panic would be
    // very unexpected.
    // Use a non-greedy match so we capture what is the href value.
    // Sample capture:
    // <a href="https://kennethreitz.org" target="_blank"> => https://github.com/requests/httpbin
    let re = Regex::new("href=\"(.*?)\"").unwrap();

    while !url_queue.is_empty() {
        // Safe to return an error, here as the queue should not be empty
        if let Some(url) = url_queue.pop_front() {
            if let Ok(body) = reqwest::get(url).await {
                //println!("body = {:?}", body);
                for cap in re.captures_iter(&body.text().await?) {
                    println!("{}", &cap[1]);
                    if let Ok(new_url) = Url::parse(&format!("{}", &cap[1])) {
                        url_queue.push_back(new_url.clone());
                        seen_urls.insert(new_url.clone());
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
