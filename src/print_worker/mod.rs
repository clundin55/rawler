use std::fmt;
use std::fmt::Display;
use std::sync::mpsc::Receiver;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrintItem<T: Display> {
    crawled_page: T,
    nested_pages: Vec<T>,
}

impl<T: Display> PrintItem<T> {
    pub fn new(crawled_page: T, nested_pages: Vec<T>) -> Self {
        PrintItem {
            crawled_page,
            nested_pages,
        }
    }
}

impl<T: Display> fmt::Display for PrintItem<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut found_urls = format!("{}", self.crawled_page).to_string();
        for page in &self.nested_pages {
            found_urls.push_str(&format!("\n {}", page));
        }

        write!(f, "{}", found_urls)
    }
}

pub async fn print_job<T>(print_queue: Receiver<PrintItem<T>>, timeout: Duration) -> !
where
    T: Display,
{
    loop {
        print_queue
            .recv_timeout(timeout)
            .and_then(|page| Ok(println!("{}", page)))
            .unwrap();
    }
}
