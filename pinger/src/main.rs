use futures::{stream, StreamExt};
use serde::Deserialize;

const MAX_CONCURRENT_REQUESTS: usize = 10;

#[tokio::main]
async fn main() {
    let f = std::fs::read_to_string("urls.json").unwrap();
    let doc: Document = serde_json::from_str(&f).unwrap();
    let results = check_urls(doc).await;
    for (name, ok) in results {
        println!("{name:20} {ok}")
    }
}

#[derive(Debug, Deserialize)]
struct Document {
    urls: Vec<Website>,
}

#[derive(Debug, Deserialize)]
struct Website {
    name: String,
    url: String,
}

async fn check_urls(doc: Document) -> Vec<(String, bool)> {
    let vec_of_futures = doc.urls.into_iter().map(|Website { name, url }| async {
        let Ok(resp) = reqwest::get(url).await else {
            return (name, false);
        };
        (name, resp.text().await.is_ok())
    });

    let stream_of_futures = stream::iter(vec_of_futures);

    stream_of_futures
        .buffer_unordered(MAX_CONCURRENT_REQUESTS)
        .collect()
        .await
}
