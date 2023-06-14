use async_trait::async_trait;

mod error;
pub mod model;

#[async_trait]
pub trait Datasource {
    async fn search_content(&self) -> Result<model::search_content::Response, error::Error>;
    async fn get_pages(&self) -> Result<model::get_pages::Response, error::Error>;
    async fn get_content(&self) -> Result<model::get_content::Response, error::Error>;

    async fn get_async_content_body(
        &self,
        id: &str,
    ) -> Result<model::get_async_content_body::Response, error::Error>;
}

pub struct ConfluenceApi<'a> {
    http_client: &'a reqwest::Client,
    api_key: String,
    base_url: String,
}

impl<'a> ConfluenceApi<'a> {
    pub fn new(
        http_client: &'a reqwest::Client,
        api_key: &str,
        username: &str,
        domain: &str,
    ) -> Self {
        let api_key = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            format!("{}:{}", username, api_key),
        );

        Self {
            http_client,
            api_key,
            base_url: String::from(domain),
        }
    }
}

#[async_trait]
impl<'a> Datasource for ConfluenceApi<'a> {
    async fn search_content(&self) -> Result<model::search_content::Response, error::Error> {
        let url = format!("{}/wiki/rest/api/search?cql=type=page", self.base_url);

        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    async fn get_pages(&self) -> Result<model::get_pages::Response, error::Error> {
        let url = format!("{}/wiki/api/v2/pages", self.base_url);

        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    async fn get_content(&self) -> Result<model::get_content::Response, error::Error> {
        let url = format!(
            "{}/wiki/rest/api/content?expand=body.atlas_doc_format",
            self.base_url
        );

        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    async fn get_async_content_body(
        &self,
        id: &str,
    ) -> Result<model::get_async_content_body::Response, error::Error> {
        let url = format!(
            "{}/wiki/rest/api/contentbody/convert/async/{}",
            self.base_url, id
        );

        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }
}
