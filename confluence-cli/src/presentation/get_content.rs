use super::command::Command;
use anyhow::Error;
use async_trait::async_trait;
use confluence_api::Datasource;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {}

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

        let response = datasource.get_content().await?;
        response.results.into_iter().for_each(|content| {
            let atlas_doc_formatted_content = &content.body.atlas_doc_format.value;
            let temp = atlas_doc_formatted_content.replace("\\\"", "^^^$$$!!!");
            let unescaped = temp.replace("\\", "");
            let json = unescaped.replace("^^^$$$!!!", "\\\"");

            println!(
                "{:#?}",
                serde_json::from_str::<confluence_api::model::atlas_doc_format::Root>(&json)
            );
        });

        Ok(())
    }
}
