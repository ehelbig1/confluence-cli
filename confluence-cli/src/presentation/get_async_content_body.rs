use super::command::Command;
use anyhow::Error;
use async_trait::async_trait;
use confluence_api::Datasource;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    id: String,
}

#[async_trait]
impl Command for Opt {
    async fn run(
        &self,
        http_client: &reqwest::Client,
        api_key: &str,
        username: &str,
        domain: &str,
    ) -> Result<(), Error> {
        let datasource = confluence_api::ConfluenceApi::new(http_client, api_key, username, domain);
        let response = datasource.get_async_content_body(&self.id).await?;

        println!("{:#?}", response);

        Ok(())
    }
}
