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
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Document(x) => x.fmt(f),
            Node::Heading(x) => x.fmt(f),
        }
    }
}

impl Node {
    /// Accept new [`Visitor`] to visit this `mdast`
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Node::Document(x) => visitor.visit_document(x),
            Node::Heading(x) => visitor.visit_heading(x),
        }
    }
}

/// [mdast](https://github.com/syntax-tree/mdast#list) visitor must implement this trait.
#[allow(unused_variables)]
pub trait Visitor {
    fn visit_document(&mut self, document: &Document) {}

    fn visit_heading(&mut self, heading: &Heading) {}
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
