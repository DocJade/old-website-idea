// time for angry clippy!

#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::correctness,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::cognitive_complexity,
    clippy::double_parens,
    clippy::len_zero,
    clippy::question_mark,
    clippy::suspicious,
    clippy::todo,
    //clippy::all  //for extra anger
)]

use std::fs;
use std::io;
use std::path::Path;

mod dcms;
use dcms::node;
use dcms::parse;
use dcms::parse::recurse_node;
use maud::html;

use crate::dcms::node::Node;

// open each file and run the processor.

fn process_files(source_dir: &Path, target_dir: &Path) -> io::Result<()> {
    if !source_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Source directory does not exist",
        ));
    }

    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
    }

    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            println!("Found a folder.");
            let target_sub_dir = target_dir.join(path.strip_prefix(source_dir).unwrap());
            println!(
                "Creating folder {} and recursing...",
                target_sub_dir.display()
            );
            process_files(&path, &target_sub_dir)?;
        } else {
            let target_file = target_dir.join(path.strip_prefix(source_dir).unwrap());

            // Check if file ends with ".dcms"
            if path.extension().and_then(|ext| ext.to_str()) == Some("dcms") {
                // Process DCMS file
                println!("{} is a DCMS file!", path.display());
                let contents = fs::read_to_string(&path)?;
                let processed_contents = process_file_content(contents).unwrap();

                // Change extension to .html
                let mut target_file_with_html_ext = target_file.clone();
                target_file_with_html_ext.set_extension("html");

                fs::write(&target_file_with_html_ext, processed_contents)?;
            } else {
                // Copy file directly if not DCMS
                println!("Copying {}...", path.display());
                fs::copy(&path, &target_file)?;
            }
        }
    }

    Ok(())
}

fn process_file_content(contents: String) -> Option<String> {
    println!("Starting site builder!");

    // split the string on newlines
    let lines: Vec<&str> = contents.split('\n').collect();

    // this is our master node, we will add children to it as we go.
    let children_start: Vec<Node> = vec![];
    let master_node: Node = Node {
        content: None,
        attributes: node::ContentAttributes::Head,
        children: Some(children_start),
    };

    // loop over each line
    for line in lines {
        // call the parser
    }

    todo!()
}

fn main() -> io::Result<()> {
    let source_dir = Path::new("pages");
    let target_dir = Path::new("site_output");

    process_files(source_dir, target_dir)?;

    Ok(())
}

#[test]
fn make_valid_header_site() {
    let master_node: Node = Node {
        content: Some("# This is a test!".to_string()),
        attributes: node::ContentAttributes::Head,
        children: None,
    };
    println!("{master_node:#?}");
    // call the parser
    let result = recurse_node(master_node);
    println!("{result:#?}");
    // now, can we make html out of it?
    // turn it into it's string
    let intermediate_step_string: String = format!("{result}");
    println!("{intermediate_step_string}");
    // now make the html.
    let html = html! {(intermediate_step_string);};
    println!("{html:#?}");
    panic!()
}
