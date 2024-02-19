use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;
use std::env;


error_chain! {
    foreign_links{
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}
#[tokio::main]
async fn main() -> Result <()> {
        let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <URL>", args[0]);
        return Ok(());
    }

}
