// time to read the dcms!

// time for a lot of regex

use regex::Regex;

use crate::node::ContentAttributes;
use crate::node::Node;

use super::find_headings::find_h_header;

#[derive(Debug)]
pub struct RegexResults {
    // we will never store anything before the regex applies,
    // if the regex does not apply at the start of the string, it is invalid.

    // if something does match, it will have a content attribute type.
    pub result_type: ContentAttributes,
    // then we have the text after the triggers are removed
    pub resulting_string: Option<String>,
    // and everything that was left over after the string.
    pub the_rest: Option<String>,
}

#[derive(Debug)]
pub struct ParseResult {
    pub node: Node,
    pub leftovers: Option<String>,
}

// search and find things!
pub fn parse_next(input: &str) -> ParseResult {
    // look for everything

    let mut result: Option<RegexResults>;

    // loop for easier breaking out / cleaner look

    #[allow(clippy::never_loop)] // yes it doesn't but this looks nice
    loop {
        // look for headers
        result = find_h_header(input);

        if result.is_some() {
            // got something!
            break;
        }

        // didnt find anything, therefore this is plaintext
        return ParseResult {
            node: Node {
                content: Some(input.to_string()),
                attributes: ContentAttributes::Plain,
                children: None,
            },
            leftovers: None,
        };
    }

    // if we got out of that loop, means we found something
    let unwrap_result = result.unwrap();
    // turn it into a node
    let new_node: Node = Node {
        content: unwrap_result.resulting_string,
        attributes: unwrap_result.result_type,
        children: None,
    };

    // return the result thing
    ParseResult {
        node: new_node,
        leftovers: unwrap_result.the_rest,
    }
}

pub fn parse_all(input: &str) -> Vec<Node> {
    let mut remaining_string: Option<String> = Some(input.to_string());
    let mut nodes: Vec<Node> = vec![];

    // run the parser till we run out of string, making sure to recurse down
    loop {
        if remaining_string.is_none() {
            // done!
            break;
        }
        // parse the next thing
        let result: ParseResult = parse_next(&remaining_string.unwrap());
        // new string is the leftovers
        remaining_string = result.leftovers;
        // save the node
        nodes.append(&mut vec![result.node])
    }
    nodes
}

#[allow(clippy::useless_let_if_seq)] // no idea what it means
pub fn recurse_node(node: Node) -> Node {
    let mut new_children: Vec<Node> = vec![];
    // does this node have any kids?
    if node.children.is_none() {
        // nope!
        // is it's type plaintext?
        if node.attributes == ContentAttributes::Plain {
            // yep! we wont mess with it.
            return node;
        };
        // not plaintext!
        // does it have content?

        if node.content.is_none() {
            // dont mess with it.
            return node;
        };

        // need to parse it!
        new_children = parse_all(&node.content.clone().unwrap());

        // now call self again
        return recurse_node(Node {
            content: None,
            attributes: node.attributes,
            children: Some(new_children),
        });
    } else {
        // looks like we have kids to loop over.
        for child in node.children.unwrap() {
            // recurse!
            new_children.append(&mut vec![recurse_node(child)]);
        }
    }
    // put the node back together
    Node {
        content: None,
        attributes: node.attributes,
        children: Some(new_children),
    }
}

// Tests!

#[test]
fn find_header_test() {
    let test_string: &str = "# this is a header";
    let result = parse_next(test_string);
    // println!("{result:#?}");
    assert_eq!(result.node.attributes, ContentAttributes::HeaderSize(1));
}

#[test]
fn find_plaintext_test() {
    let test_string: &str = "blah blah blah";
    let result = parse_next(test_string);
    // println!("{result:#?}");
    assert_eq!(result.node.attributes, ContentAttributes::Plain);
}

#[test]
fn recursive_parse_test_empty() {
    let master_node: Node = Node {
        content: None,
        attributes: ContentAttributes::Head,
        children: None,
    };
    let result: Node = recurse_node(master_node);
    println!("{result:#?}");
    panic!()
}

#[test]
fn recursive_parse_test() {
    let master_node: Node = Node {
        content: Some("# this is a header!".to_string()),
        attributes: ContentAttributes::Head,
        children: None,
    };
    let result: Node = recurse_node(master_node);
    println!("{result:#?}");
    panic!()
}
