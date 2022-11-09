use std::path::PathBuf;

pub struct Document {
    pub elements: Vec<Element>,
}

pub enum Element {
    Heading(Heading),
    Paragraph(Paragraph),
    BlockQuote(Document),
    List(List),
    CodeBlock(CodeBlock),
    Image(Image),
    HorizontalRule,
    Link(Link),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Heading {
    pub level: u8,
    pub content: String,
}

pub struct Paragraph {
    pub segments: Vec<Segment>,
}

pub struct Segment {
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub content: String,
}

pub struct List {
    pub ordered: bool,
    pub items: Vec<Paragraph>,
}

pub struct Item {
    pub content: Document,
}

pub struct CodeBlock {
    pub language: Option<String>,
    pub content: String,
}

pub struct Image {
    pub alt: String,
    pub path: PathBuf,
}

pub struct Link {
    pub anchor: Anchor,
    pub url: Address,
    pub title: String,
}

pub enum Anchor {
    Text(String),
    Image(Image),
}

pub enum Address {
    Url(String),
    Email(String),
}
