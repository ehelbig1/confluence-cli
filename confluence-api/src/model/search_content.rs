use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Response {
    pub results: Vec<Result>,
    pub start: usize,
    pub limit: usize,
    pub total_size: Option<usize>,
    pub cql_query: Option<String>,
    pub search_duration: Option<usize>,
    pub archive_result_count: Option<usize>,

    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Links {}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Result {
    pub content: Content,
    pub user: Option<User>,
    pub space: Option<Space>,
    pub title: String,
    pub excerpt: String,
    pub url: String,
    pub result_parent_container: Option<ResultParentContainer>,
    pub result_global_container: Option<ResultGlobalContainer>,
    pub breadcrumbs: Vec<Breadcrumb>,
    pub entity_type: Option<String>,
    pub icon_css_class: Option<String>,
    pub last_modified: Option<String>,
    pub friendly_last_modified: Option<String>,
    pub score: f32,
}

#[derive(Debug, Deserialize)]
pub struct ResultParentContainer {}

#[derive(Debug, Deserialize)]
pub struct ResultGlobalContainer {}

#[derive(Debug, Deserialize)]
pub struct Breadcrumb {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub id: String,
    pub r#type: String,
    pub status: String,
    pub title: String,
    pub space: Option<Space>,
    pub history: Option<History>,
    pub version: Option<Version>,
    pub ancestors: Option<Vec<String>>,
    pub operations: Option<Vec<Operation>>,
    pub children: Option<Children>,
    pub child_types: ChildTypes,
    pub descendants: Option<Descendants>,
    pub container: Option<Container>,
    pub body: Option<Body>,
    pub restrictions: Restrictions,
    pub metadata: Option<Metadata>,
    pub macro_rendered_output: MacroRenderedOutput,
    pub extensions: Option<Extensions>,

    #[serde(rename = "_expandable")]
    pub expandable: Expandable,

    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct History {}

#[derive(Debug, Deserialize)]
pub struct Version {}

#[derive(Debug, Deserialize)]
pub struct Operation {}

#[derive(Debug, Deserialize)]
pub struct Children {}

#[derive(Debug, Deserialize)]
pub struct ChildTypes {}

#[derive(Debug, Deserialize)]
pub struct Descendants {}

#[derive(Debug, Deserialize)]
pub struct Container {}

#[derive(Debug, Deserialize)]
pub struct Body {}

#[derive(Debug, Deserialize)]
pub struct Restrictions {}

#[derive(Debug, Deserialize)]
pub struct Metadata {}

#[derive(Debug, Deserialize)]
pub struct MacroRenderedOutput {}

#[derive(Debug, Deserialize)]
pub struct Extensions {}

#[derive(Debug, Deserialize)]
pub struct Expandable {}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct User {
    pub r#type: String,
    pub username: String,
    pub user_key: String,
    pub account_id: String,
    pub email: String,
    pub public_name: String,
    pub profile_picture: ProfilePicture,
    pub display_name: String,
    pub time_zone: String,
    pub is_external_collaborator: bool,
    pub external_collaborator: bool,
    pub operations: Vec<Operation>,
    pub details: Details,
    pub personal_space: PersoanlSpace,

    #[serde(rename = "_expandable")]
    pub expandable: Expandable,

    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct ProfilePicture {}

#[derive(Debug, Deserialize)]
pub struct Details {}

#[derive(Debug, Deserialize)]
pub struct PersoanlSpace {}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Space {
    pub id: String,
    pub key: String,
    pub name: String,
    pub icon: Icon,
    pub description: Description,
    pub homepage: Homepage,
    pub r#type: String,
    pub metadata: Metadata,
    pub operations: Vec<Operation>,
    pub permissions: Vec<Permission>,
    pub status: String,
    pub settings: Settings,
    pub theme: Theme,
    pub look_and_feel: LookAndFeel,
    pub history: History,

    #[serde(rename = "_expandable")]
    pub expandable: Expandable,

    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Icon {}

#[derive(Debug, Deserialize)]
pub struct Description {}

#[derive(Debug, Deserialize)]
pub struct Homepage {}

#[derive(Debug, Deserialize)]
pub struct Permission {}

#[derive(Debug, Deserialize)]
pub struct Settings {}

#[derive(Debug, Deserialize)]
pub struct Theme {}

#[derive(Debug, Deserialize)]
pub struct LookAndFeel {}
