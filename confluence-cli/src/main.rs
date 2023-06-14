use anyhow::Error;
use std::{env, time};
use structopt::StructOpt;

mod presentation;
use presentation::command::Command;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, env = "CONFLUENCE_API_KEY", hide_env_values = true)]
    api_key: String,

    #[structopt(long, env = "CONFLUENCE_USERNAME", hide_env_values = true)]
    username: String,

    #[structopt(long, env = "CONFLUENCE_DOMAIN", hide_env_values = true)]
    domain: String,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    Search(presentation::search::Opt),
    GetPages(presentation::get_pages::Opt),
    GetContent(presentation::get_content::Opt),
    GetAsyncContentBody(presentation::get_async_content_body::Opt),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("Program started!");

    let opt = Opt::from_args();
    let api_key = &opt.api_key;
    let username = &opt.username;
    let domain = &opt.domain;
    let http_client = reqwest::ClientBuilder::new()
        .timeout(time::Duration::from_secs(5))
        .build()?;

    match &opt.subcommand {
        Subcommand::Search(opt) => opt.run(&http_client, api_key, username, domain).await?,
        Subcommand::GetPages(opt) => opt.run(&http_client, api_key, username, domain).await?,
        Subcommand::GetAsyncContentBody(opt) => {
            opt.run(&http_client, api_key, username, domain).await?
        }
        Subcommand::GetContent(opt) => opt.run(&http_client, api_key, username, domain).await?,
    };

    Ok(())
}
