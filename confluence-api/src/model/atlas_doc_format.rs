use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Root {
    pub r#type: String,
    pub content: Vec<Content>,
    pub version: usize,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Content {
    Panel(Panel),
    Paragraph(Paragraph),
    Text(Text),
    MediaSingle(MediaSingle),
    Media(Media),
    File(File),
    LayoutSection(LayoutSection),
    LayoutColumn(LayoutColumn),
    Rule,
    Heading(Heading),
    Strong,
    Em,
    Table(Table),
    TableRow(TableRow),
    TableCell(TableCell),
    TableHeader(TableHeader),
    Alignment(Alignment),
    HardBreak,
    Link(Link),
    Extension,
    InlineExtension,
    Image,
    TaskList,
    TaskItem,
    CodeBlock,
    BulletList,
    Blockquote,
    OrderedList,
    DecisionList,
    Date,
    Mention,
    InlineCard,
    Placeholder,
    TextColor,
    Indentation,
    BodiedExtension,
    Emoji,
}

#[derive(Deserialize, Debug)]
pub struct Panel {
    pub attrs: PanelAttrs,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PanelAttrs {
    pub panel_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Paragraph {
    pub content: Option<Vec<Content>>,
    pub marks: Option<Vec<Content>>,
}

#[derive(Deserialize, Debug)]
pub struct Text {
    pub text: String,
    pub marks: Option<Vec<Content>>,
}

#[derive(Deserialize, Debug)]
pub struct Alignment {
    pub attrs: AlignmentAttrs,
}

#[derive(Deserialize, Debug)]
pub struct AlignmentAttrs {
    pub align: String,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub attrs: LinkAttrs,
}

#[derive(Deserialize, Debug)]
pub struct LinkAttrs {
    pub href: String,
}

#[derive(Deserialize, Debug)]
pub struct MediaSingle {
    pub attrs: MediaSingleAttrs,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct MediaSingleAttrs {
    pub layout: String,
    pub content: Option<Vec<Content>>,
    pub width: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub attrs: Box<Content>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    #[serde(rename = "__fileSize")]
    pub file_size: Option<usize>,

    #[serde(rename = "__fileMimeType")]
    pub file_mime_type: Option<String>,

    #[serde(rename = "__fileName")]
    pub file_name: Option<String>,

    pub width: usize,
    pub height: usize,
    pub id: String,
    pub collection: String,
}

#[derive(Deserialize, Debug)]
pub struct LayoutSection {
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct LayoutColumn {
    pub attrs: LayoutColumnAttrs,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct LayoutColumnAttrs {
    pub width: f32,
}

#[derive(Deserialize, Debug)]
pub struct Heading {
    pub attrs: HeadingAttrs,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct HeadingAttrs {
    pub level: usize,
}

#[derive(Deserialize, Debug)]
pub struct Table {
    pub attrs: Option<TableAttrs>,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct TableAttrs {
    #[serde(rename = "__autoSize")]
    pub auto_size: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct TableRow {
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct TableCell {
    pub attrs: TableCellAttrs,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct TableCellAttrs {
    pub colspan: usize,
    pub rowspan: usize,
}

#[derive(Deserialize, Debug)]
pub struct TableHeader {
    pub attrs: TableHeaderAttrs,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct TableHeaderAttrs {
    pub colspan: usize,
    pub rowspan: usize,
}
