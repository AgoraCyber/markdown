use std::{fmt::Debug, slice::Iter};

use thiserror::Error;

/// `mdast` associated error type.
#[derive(Error, Debug)]
pub enum AstError {}

/// `mdast` associated [Result] type.
pub type AstResult<T> = Result<T, AstError>;

/// Flow content represent the sections of document.
pub trait FlowContent {}

/// List content represent the items in a list.
pub trait ListContent {}

/// Phrasing content represent the text in a document, and its markup.
pub trait PhrasingContent {}

/// [mdast](https://github.com/syntax-tree/mdast#list) variant type.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "type")
)]
pub enum Node {
    Document(Document),
    Heading(Heading),
    ThematicBreak(ThematicBreak),
    Blockquote(Blockquote),
    List(List),
    ListItem(ListItem),
    Code(Code),
    Definition(Definition),
    Text(Text),
    Emphasis(Emphasis),
    Strong(Strong),
    InlineCode(InlineCode),
    Break(Break),
    Link(Link),
    LinkReference(LinkReference),
    Image(Image),
    ImageReference(ImageReference),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Document(x) => x.fmt(f),
            Node::Heading(x) => x.fmt(f),
            Node::ThematicBreak(x) => x.fmt(f),
            Node::Blockquote(x) => x.fmt(f),
            Node::List(x) => x.fmt(f),
            Node::ListItem(x) => x.fmt(f),
            Node::Code(x) => x.fmt(f),
            Node::Definition(x) => x.fmt(f),
            Node::Text(x) => x.fmt(f),
            Node::Emphasis(x) => x.fmt(f),
            Node::Strong(x) => x.fmt(f),
            Node::InlineCode(x) => x.fmt(f),
            Node::Break(x) => x.fmt(f),
            Node::Link(x) => x.fmt(f),
            Node::LinkReference(x) => x.fmt(f),
            Node::Image(x) => x.fmt(f),
            Node::ImageReference(x) => x.fmt(f),
        }
    }
}

impl Node {
    /// Accept new [`Visitor`] to visit this `mdast`
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Node::Document(x) => visitor.visit_document(x),
            Node::Heading(x) => visitor.visit_heading(x),
            Node::ThematicBreak(x) => visitor.visit_thematic_break(x),
            Node::Blockquote(x) => visitor.visit_blockquote(x),
            Node::List(x) => visitor.visit_list(x),
            Node::ListItem(x) => visitor.visit_list_item(x),
            Node::Code(x) => visitor.visit_code(x),
            Node::Definition(x) => visitor.visit_definition(x),
            Node::Text(x) => visitor.visit_text(x),
            Node::Emphasis(x) => visitor.visit_emphasis(x),
            Node::Strong(x) => visitor.visit_strong(x),
            Node::InlineCode(x) => visitor.visit_inline_code(x),
            Node::Break(x) => visitor.visit_break(x),
            Node::Link(x) => visitor.visit_link(x),
            Node::LinkReference(x) => visitor.visit_link_reference(x),
            Node::Image(x) => visitor.visit_image(x),
            Node::ImageReference(x) => visitor.visit_image_reference(x),
        }
    }
}

/// [mdast](https://github.com/syntax-tree/mdast#list) visitor must implement this trait.
#[allow(unused_variables)]
pub trait Visitor {
    fn visit_document(&mut self, document: &Document) {}

    fn visit_heading(&mut self, heading: &Heading) {}

    fn visit_thematic_break(&mut self, thematic_break: &ThematicBreak) {}

    fn visit_blockquote(&mut self, blockquote: &Blockquote) {}

    fn visit_list(&mut self, node: &List) {}

    fn visit_list_item(&mut self, node: &ListItem) {}

    fn visit_code(&mut self, node: &Code) {}

    fn visit_definition(&mut self, node: &Definition) {}

    fn visit_text(&mut self, node: &Text) {}

    fn visit_emphasis(&mut self, node: &Emphasis) {}

    fn visit_strong(&mut self, node: &Strong) {}

    fn visit_inline_code(&mut self, node: &InlineCode) {}

    fn visit_break(&mut self, node: &Break) {}

    fn visit_link(&mut self, node: &Link) {}

    fn visit_link_reference(&mut self, node: &LinkReference) {}

    fn visit_image(&mut self, node: &Image) {}

    fn visit_image_reference(&mut self, node: &ImageReference) {}
}

/// Parent (UnistParent) represents an abstract interface in
/// mdast containing other nodes (said to be children).
pub trait Parent<Child>
where
    Child: Into<Node>,
{
    type Iter<'a>: Iterator<Item = &'a Node>
    where
        Self: 'a;

    /// Addd one child node.
    fn add_child(&mut self, node: Child) -> AstResult<()>;

    /// Removes and returns the child [Node] at position `index`
    fn remove_at(&mut self, index: usize) -> Node;

    /// Return an iterator over children slice.
    fn iter<'a>(&'a self) -> Self::Iter<'a>;
}

macro_rules! parent {
    ($node_name:ident) => {
        impl<Child> Parent<Child> for $node_name
        where
            Child: Into<Node>,
        {
            type Iter<'a> = Iter<'a, Node>;

            fn add_child(&mut self, node: Child) -> AstResult<()> {
                self.children.push(node.into());
                Ok(())
            }

            fn iter<'a>(&'a self) -> Self::Iter<'a> {
                self.children.iter()
            }

            fn remove_at(&mut self, index: usize) -> Node {
                self.children.remove(index)
            }
        }
    };
    ($node_name:ident,$content_type: ident) => {
        impl<Child> Parent<Child> for $node_name
        where
            Child: Into<Node> + $content_type,
        {
            type Iter<'a> = Iter<'a, Node>;

            fn add_child(&mut self, node: Child) -> AstResult<()> {
                self.children.push(node.into());
                Ok(())
            }

            fn iter<'a>(&'a self) -> Self::Iter<'a> {
                self.children.iter()
            }

            fn remove_at(&mut self, index: usize) -> Node {
                self.children.remove(index)
            }
        }
    };
}

/// Document.
///
/// ```markdown
/// > | a
///     ^
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "document")
)]
pub struct Document {
    pub children: Vec<Node>,
}

parent!(Document);

/// Paragraph (Parent) represents a unit of discourse dealing with a particular point or idea.
/// For example, the following markdown:
/// ```markdown
/// Alpha bravo charlie.
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "paragraph")
)]
pub struct Paragraph {
    pub children: Vec<Node>,
}

parent!(Paragraph, PhrasingContent);

impl FlowContent for Paragraph {}

/// Heading (Parent) represents a heading of a section.
/// Heading can be used where flow content is expected.
/// Its content model is phrasing content.
///
/// ```markdown
/// # Alpha
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "heading")
)]
pub struct Heading {
    /// Children node list.
    pub children: Vec<Node>,
    /// A depth field must be present.
    /// A value of 1 is said to be the highest rank and 6 the lowest.
    pub depth: usize,
}

impl Heading {
    /// Create new [`Heading`] instance with provided `depth`
    pub fn new(depth: usize) -> Self {
        assert!(
            depth > 0 && depth < 7,
            "A value of 1 is said to be the highest rank and 6 the lowest"
        );

        Heading {
            children: Default::default(),
            depth,
        }
    }
}

parent!(Heading, PhrasingContent);

impl FlowContent for Heading {}

/// ThematicBreak (Node) represents a thematic break,
/// such as a scene change in a story,
/// a transition to another topic, or a new document.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "thematicbreak")
)]
pub struct ThematicBreak {}

impl FlowContent for ThematicBreak {}

/// Blockquote (Parent) represents a section quoted from somewhere else.
/// ```markdown
/// # Alpha
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "blockquote")
)]
pub struct Blockquote {
    /// Children node list.
    pub children: Vec<Node>,
}

parent!(Blockquote, PhrasingContent);

impl FlowContent for Blockquote {}

/// List (Parent) represents a list of items.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "list")
)]
pub struct List {
    /// Children node list.
    pub children: Vec<Node>,
}

parent!(List, ListContent);

impl FlowContent for List {}

/// ListItem (Parent) represents an item in a List.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "list")
)]
pub struct ListItem {
    /// Children node list.
    pub children: Vec<Node>,
}

parent!(ListItem, FlowContent);

impl ListContent for ListItem {}

/// Code (Literal) represents a block of preformatted text, such as ASCII art or computer code.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "code")
)]
pub struct Code {
    /// Literal data.
    pub value: String,
    /// [`Option`] field to indicate code language.
    pub lang: Option<String>,
    /// Meta data for code language.
    pub meta: Option<String>,
}

impl FlowContent for Code {}

/// Code (Literal) represents a block of preformatted text, such as ASCII art or computer code.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "definition")
)]
pub struct Definition {
    /// Children node list.
    pub children: Vec<Node>,
    /// An identifier field must be present. It can match another node.
    /// identifier is a source value: character escapes and character
    /// references are not parsed. Its value must be normalized.
    pub identifier: String,
    /// A label field can be present.
    /// label is a string value: it works just like title on a link or a lang on
    /// code: character escapes and character references are parsed.
    pub label: Option<String>,
    /// A url field must be present. It represents a URL to the referenced resource.
    pub url: String,
    /// A title field can be present.
    /// It represents advisory information for the resource,
    /// such as would be appropriate for a tooltip.
    pub title: Option<String>,
}

impl FlowContent for Definition {}

/// Text (Literal) represents everything that is just text.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "text")
)]
pub struct Text {
    /// Text literal value
    pub value: String,
}
/// Text can be used where phrasing content is expected.
/// Its content is represented by its value field.
impl PhrasingContent for Text {}

/// Emphasis (Parent) represents stress emphasis of its contents.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "emphasis")
)]
pub struct Emphasis {
    /// Children node list.
    pub children: Vec<Node>,
}

parent!(Emphasis, PhrasingContent);

/// Emphasis can be used where phrasing content is expected.
/// Its content model is phrasing content.
impl PhrasingContent for Emphasis {}

/// Strong (Parent) represents strong importance, seriousness, or urgency for its contents.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "strong")
)]
pub struct Strong {
    /// Children node list.
    pub children: Vec<Node>,
}

parent!(Strong, PhrasingContent);

/// Strong can be used where phrasing content is expected.
/// Its content model is phrasing content.
impl PhrasingContent for Strong {}

/// InlineCode (Literal) represents a fragment of computer code, such as a file name,
/// computer program, or anything a computer could parse.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "inlinecode")
)]
pub struct InlineCode {
    /// Text literal value
    pub value: String,
}
/// InlineCode can be used where phrasing content is expected.
/// Its content is represented by its value field.
impl PhrasingContent for InlineCode {}

/// InlineCode (Literal) represents a fragment of computer code, such as a file name,
/// computer program, or anything a computer could parse.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "break")
)]
pub struct Break {}
/// Break can be used where phrasing content is expected.
/// Its content is represented by its value field.
impl PhrasingContent for Break {}

/// Link (Parent) represents a hyperlink.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "link")
)]
pub struct Link {
    /// Children node list.
    pub children: Vec<Node>,
    /// A url field must be present. It represents a URL to the referenced resource.
    pub url: String,
    /// A title field can be present.
    /// It represents advisory information for the resource,
    /// such as would be appropriate for a tooltip.
    pub title: Option<String>,
}

parent!(Link, PhrasingContent);

/// Link can be used where phrasing content is expected.
/// Its content model is phrasing content.
impl PhrasingContent for Link {}

/// Link (Parent) represents a hyperlink.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "link")
)]
pub struct LinkReference {
    /// Children node list.
    pub children: Vec<Node>,
    /// An identifier field must be present. It can match another node.
    /// identifier is a source value: character escapes and character
    /// references are not parsed. Its value must be normalized.
    pub identifier: String,
    /// A label field can be present.
    /// label is a string value: it works just like title on a link or a lang on
    /// code: character escapes and character references are parsed.
    pub label: Option<String>,
    /// A referenceType field must be present. Its value must be a referenceType.
    /// It represents the explicitness of the reference.
    pub reference_type: String,
}

parent!(LinkReference, PhrasingContent);

/// LinkReference can be used where phrasing content is expected.
/// Its content model is phrasing content.
impl PhrasingContent for LinkReference {}

/// Image (Node) represents an image.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "image")
)]
pub struct Image {
    /// A url field must be present. It represents a URL to the referenced resource.
    pub url: String,
    /// A title field can be present.
    /// It represents advisory information for the resource,
    /// such as would be appropriate for a tooltip.
    pub title: Option<String>,
    /// An alt field should be present.
    /// It represents equivalent content for environments
    /// that cannot represent the node as intended.
    pub alt: Option<String>,
}

/// Image can be used where phrasing content is expected.
/// Its content model is phrasing content.
impl PhrasingContent for Image {}

/// Image (Node) represents an image.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", rename = "strong")
)]
pub struct ImageReference {
    /// A url field must be present. It represents a URL to the referenced resource.
    pub url: String,
    /// A title field can be present.
    /// It represents advisory information for the resource,
    /// such as would be appropriate for a tooltip.
    pub title: Option<String>,
    /// An alt field should be present.
    /// It represents equivalent content for environments
    /// that cannot represent the node as intended.
    pub alt: Option<String>,
}

/// ImageReference can be used where phrasing content is expected.
/// Its content model is phrasing content.
impl PhrasingContent for ImageReference {}
