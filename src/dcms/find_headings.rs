use regex::Regex;

use super::{node::ContentAttributes, parse::RegexResults};

pub fn find_h_header(input: &str) -> Option<RegexResults> {
    // first see if the string starts with an "#" for early exit

    // make sure the string isnt empty
    if input.is_empty() {
        // cant work on an empty string smh
        return None;
    };

    if &input[0..1] != "#" {
        // no hash, we're done.
        return None;
    }

    // there is a hash! run the regex.
    let regex: Regex = Regex::new(r"^(#+) (.+)").unwrap();

    let pre_results = regex.captures(&input);

    // did we actually get anything?
    pre_results.as_ref()?;

    // yes, pull it out
    let results = pre_results.unwrap();

    // now pull out the number of hashes, and the text.

    let num = results.get(1).unwrap().len();
    let text = results.get(2).unwrap().as_str().to_string();

    // build the result and get outta here

    Some(RegexResults {
        result_type: ContentAttributes::HeaderSize(num),
        resulting_string: Some(text),
        the_rest: None,
    })
}

#[test]
fn header_sizes_and_text() {
    // can we find headers and their text?
    let hashes: Vec<String> = vec![
        "#".to_string(),
        "##".to_string(),
        "###".to_string(),
        "####".to_string(),
    ];

    let test_text: String = "Howdy!".to_string();

    for hash in hashes {
        let test_string = format!("{hash} {test_text}");
        let hash_len = hash.len();
        let result = find_h_header(&test_string).unwrap();
        // pull it apart
        assert_eq!(result.resulting_string.unwrap(), test_text);
        assert_eq!(result.the_rest, None);
        assert_eq!(result.result_type, ContentAttributes::HeaderSize(hash_len));
    }
}

#[test]
fn header_empty() {
    assert!(find_h_header("# ").is_none());
    assert!(find_h_header("#").is_none());
    assert!(find_h_header("").is_none());
}
