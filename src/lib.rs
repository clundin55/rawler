mod crawl_worker;
mod print_worker;

use crawl_worker::*;
use print_worker::*;
use std::collections::VecDeque;
use std::sync::mpsc::channel;
use std::time::Duration;
use url::Url;

pub type CrawlDepth = u32;
pub type CrawlError = Box<dyn std::error::Error>;

pub async fn crawl(root_url: Url, depth: CrawlDepth, timeout: Duration) -> Result<(), CrawlError> {
    let root_page = CrawledItem::new(root_url, 0);
    let mut pages_to_crawl = VecDeque::new();
    pages_to_crawl.push_back(root_page);

    let (page_sender, page_receiver) = channel();
    let (print_sender, print_receiver) = channel();

    // Spawn the printing thread. This thread will need a timeout or else it will fall over before
    // we start posting print jobs to it.
    tokio::spawn(async move {
        print_job(print_receiver, timeout).await;
    });

    // Keep crawling pages until the depth limit is reached for all paths
    while !pages_to_crawl.is_empty() {
        if let Some(page) = pages_to_crawl.pop_front() {
            if *page.get_depth() < depth {
                let page_sender = page_sender.clone();
                let print_sender = print_sender.clone();
                tokio::spawn(async move {
                    if let Ok(children) = crawl_job(page.clone(), page_sender).await {
                        print_sender.send(PrintItem::new(page, children));
                    }
                });
            }
        }

        // Push new pages back into the queue.
        page_receiver
            .recv_timeout(timeout)
            .and_then(|new_job| Ok(pages_to_crawl.push_back(new_job)))?;
    }

    Ok(())
}
