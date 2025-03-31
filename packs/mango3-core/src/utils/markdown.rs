use pulldown_cmark::html::push_html;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use regex::Captures;

use crate::constants::{BLACKLISTED_HASHTAGS, REGEX_FIND_HASHTAGS};
use crate::hashtag_has_lookaround;

pub fn parse_html(input: &str, enable_links: bool) -> String {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let input = REGEX_FIND_HASHTAGS.replace_all(input, |captures: &Captures| {
        let match_ = captures.name("name").expect("Could not get match");
        let name = match_.as_str();

        if !BLACKLISTED_HASHTAGS.contains(&name) && hashtag_has_lookaround(input, match_) {
            format!("[#{name}](/hashtags/{name})")
        } else {
            format!("#{name}")
        }
    });

    let parser = Parser::new_ext(&input, options).filter(|event| match event {
        Event::Start(Tag::Heading {
            level: HeadingLevel::H1,
            ..
        })
        | Event::End(TagEnd::Heading(HeadingLevel::H1)) => false,
        Event::Start(Tag::HtmlBlock) | Event::End(TagEnd::HtmlBlock) | Event::Html(_) | Event::InlineHtml(_) => false,
        Event::Start(Tag::Link { .. }) | Event::End(TagEnd::Link) => enable_links,
        _ => true,
    });

    let mut html_output = String::new();

    push_html(&mut html_output, parser);

    html_output
}
