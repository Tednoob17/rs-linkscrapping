use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links{
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}
#[tokio::main]
async fn main() -> Result <()> {

        let res = reqwest::get(
            "https://accelerator.github.com/"
        ).await?
        .text()
        .await?;

        println!("Links on web page 'accelerator.github.com' : ");

        Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n|n.attr("href"))
        .for_each(|x| println!("{}", x));

        Ok(())
}
