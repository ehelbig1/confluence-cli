use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub results: Vec<Result>,

    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub id: usize,
    pub status: String,
    pub title: String,
    pub space_id: usize,
    pub parent_id: Option<usize>,
    pub parent_type: Option<String>,
    pub author_id: Option<String>,
    pub created_at: String,
    pub version: Version,
    pub body: Body,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub next: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub create_at: Option<String>,
    pub message: String,
    pub number: usize,
    pub minor_edit: bool,
    pub author_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Body {
    pub storage: Option<Storage>,
    pub atlas_doc_format: Option<AtlasDocFormat>,
}

#[derive(Debug, Deserialize)]
pub struct Storage {}

#[derive(Debug, Deserialize)]
pub struct AtlasDocFormat {}
