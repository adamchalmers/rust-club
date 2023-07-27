use serde::Deserialize;

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
    let mut results: Vec<_> = Default::default();
    for Website { name, url } in doc.urls {
        let ok = match reqwest::get(url).await {
            Ok(resp) => resp.text().await.is_ok(),
            Err(_) => false,
        };
        results.push((name, ok));
    }
    results
}
