// recurse? i didn't curse her in the first place!

use std::fmt;

#[derive(Default, Debug)]
pub struct Node {
    // Each node can have text inside of it, which is attempted to be parsed.
    pub content: Option<String>,
    pub attributes: ContentAttributes,
    // Optionally, this Vec might have children.
    pub children: Option<Vec<Node>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ContentAttributes {
    // Details about the text contained.
    // If nothing is set, the text hasn't been evaluated.
    Unevaluated,
    Head,              // The master node.
    Plain,             // just standard text
    Italic,            // _ or * or ***
    Bold,              // ** or ***
    Underline,         // __
    HeaderSize(usize), // #
    Link(String),      // []() contains the destination.
}

// Set up defaults
impl Default for ContentAttributes {
    fn default() -> Self {
        Self::Unevaluated
    }
}

// Implement the 'Display' trait to allow string formatting for Node
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Determine how to display the content based on the attribute

        // if there are any attributes besides plain, there should be no content
        // UNLESS THERE ARE NO CHILDREN

        if self.attributes != ContentAttributes::Plain && self.children.is_some() {
            // println!("{:?}",self.attributes);
            assert!(self.content.is_none());
        }

        let mut content: String; // this will be either the actual content string, or the post-evaluation of the children.

        // Recursively display children if they exist, and panic if there is also content
        if let Some(ref children) = self.children {
            // is there content present at the same time as children? there should not be. EVER
            assert!(self.content.is_none());

            content = String::new();
            for child in children {
                content.push_str(format!("{child} ").as_str());
            }

            // make sure there isnt untouched text!
            assert!(self.content.is_none());

            // since we need to run the outer shell, we must now remove the old quotes at the beginning and end

            // make sure quotes didnt sneak in here at the wrong time

            // if this was plaintext, then we can return now.
            if self.attributes == ContentAttributes::Plain {
                // we are done! no need to recurse.
                write!(f, "{content}")?;
                return Ok(());
            }

            // now make a new node based on the current one, so we can run the formatter on that.

            let internal_attributes: ContentAttributes = self.attributes.clone();

            let internal_node: Self = Self {
                content: Some(content),
                attributes: internal_attributes,
                children: None,
            };

            // apply the attribute

            write!(f, "{internal_node}")?;

            return Ok(()); // Indicate successful formatting
        }

        // we should have content now, pull it out
        let content: String = self.content.clone().unwrap();

        match &self.attributes {
            ContentAttributes::Unevaluated => panic!("Tried to display an unevaluated string!"),
            ContentAttributes::Head => write!(f, "div {{ {content}}}")?, // the master node
            ContentAttributes::Plain => write!(f, "{content}")?,
            ContentAttributes::Italic => write!(f, "i {{ {content}}}")?,
            ContentAttributes::Bold => write!(f, "b {{ {content}}}")?,
            ContentAttributes::Underline => write!(f, "u {{ {content}}}")?,
            ContentAttributes::HeaderSize(size) => write!(f, "h{size} {{ {content}}}")?,
            ContentAttributes::Link(url) => write!(f, "a {{ href=\"{url}\" {content}}}")?,
        }
        Ok(())
    }
}

#[test]
#[should_panic]
fn empty_node() {
    // this should panic
    let test_node: Node = Node::default();
    format!("{test_node}");
}

#[test]
#[should_panic]
fn node_header_without_child() {
    let test_node: Node = Node {
        content: Some("this is a test!".to_string()),
        attributes: ContentAttributes::HeaderSize(1),
        ..Default::default()
    };
    assert_eq!("h1 {\"this is a test!\"}", format!("{test_node}"));
}

#[test]
fn node_header_with_child() {
    let test_node: Node = Node {
        content: None,
        attributes: ContentAttributes::HeaderSize(1),
        children: Some(vec![Node {
            content: Some("this is a test!".to_string()),
            attributes: ContentAttributes::Plain,
            children: None,
        }]),
    };
    assert_eq!("h1 { \"this is a test!\" }", format!("{test_node}"));
}

#[test]
fn node_plain_no_children() {
    let node_with_children = Node {
        content: None,
        attributes: ContentAttributes::Plain,
        children: Some(vec![
            Node {
                content: Some(String::from("node 1")),
                attributes: ContentAttributes::Plain,
                children: None,
            },
            Node {
                content: Some(String::from("node 2")),
                attributes: ContentAttributes::Plain,
                children: None,
            },
            Node {
                content: Some(String::from("node 3")),
                attributes: ContentAttributes::Plain,
                children: None,
            },
        ]),
    };
    // println!("{node_with_children}");
    // println!("{}", format!("{node_with_children}").contains('\\'));
    assert_eq!(
        "\"node 1\" \"node 2\" \"node 3\" ",
        format!("{node_with_children}")
    );
}

#[test]
fn italic_header() {
    // can we make an italic header?
    let expected: String = "h1 { i { \"test\" } }".to_string();

    // now make that funky node
    let node_structure: Node = Node {
        content: None,
        attributes: ContentAttributes::HeaderSize(1),
        children: Some(vec![Node {
            content: None,
            attributes: ContentAttributes::Italic,
            children: Some(vec![Node {
                content: Some("test".to_string()),
                attributes: ContentAttributes::Plain,
                children: None,
            }]),
        }]),
    };

    // does it work?

    assert_eq!(expected, format!("{node_structure}"));
}

#[test]
fn bold_italic_header_with_a_link() {
    // jesus christ
    let expected: String = "h1 { i { b { a { href=\"example.com\" \"hi\" } } } }".to_string();

    // lets do this.
    let node_structure: Node = Node {
        content: None,
        attributes: ContentAttributes::HeaderSize(1),
        children: Some(vec![Node {
            content: None,
            attributes: ContentAttributes::Italic,
            children: Some(vec![Node {
                content: None,
                attributes: ContentAttributes::Bold,
                children: Some(vec![Node {
                    content: None,
                    attributes: ContentAttributes::Link("example.com".to_string()),
                    children: Some(vec![Node {
                        content: Some("hi".to_string()),
                        attributes: ContentAttributes::Plain,
                        children: None,
                    }]),
                }]),
            }]),
        }]),
    };

    // does it work?

    assert_eq!(expected, format!("{node_structure}"));
}
