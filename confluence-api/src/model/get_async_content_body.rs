use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub value: String,
    pub representation: String,
    pub render_task_id: String,
    pub error: String,
    pub status: String,
    pub embedded_content: Vec<EmbeddedContent>,
    pub webresource: WebResource,
    pub mdeia_token: MediaToken,

    #[serde(rename = "_expandable")]
    pub expandable: Expandable,

    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddedContent {}

#[derive(Debug, Deserialize)]
pub struct WebResource {}

#[derive(Debug, Deserialize)]
pub struct MediaToken {}

#[derive(Debug, Deserialize)]
pub struct Expandable {}

#[derive(Debug, Deserialize)]
pub struct Links {}
