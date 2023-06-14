use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub results: Vec<Result>,

    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    pub id: String,
    pub r#type: String,
    pub body: Body,
}

#[derive(Debug, Deserialize)]
pub struct Links {}

#[derive(Debug, Deserialize)]
pub struct Body {
    pub atlas_doc_format: AtlasDocFormat,
}

#[derive(Debug, Deserialize)]
pub struct AtlasDocFormat {
    pub value: String,
}
