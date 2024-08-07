#![deny(warnings)]

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[tokio::main]
async fn main() -> Result<(), rquest::Error> {
    // Some simple CLI args requirements...
    let url = if let Some(url) = std::env::args().nth(1) {
        url
    } else {
        println!("No CLI URL provided, using default.");
        "https://hyper.rs".into()
    };

    eprintln!("Fetching {:?}...", url);

    // rquest::get() is a convenience function.
    //
    // In most cases, you should create/build a rquest::Client and reuse
    // it for all requests.
    let res = rquest::get(url).await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{}", body);

    Ok(())
}
