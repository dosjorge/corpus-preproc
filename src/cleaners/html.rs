use crate::normalizers::character::LINE_SEPARATOR;
use clap::Parser;
use kuchiki::traits::*;
use kuchiki::{ElementData, NodeDataRef};
use serde::{Deserialize, Serialize};

use crate::normalizers::character::clean_control;

pub const DEFAULT_DELETE_SELECTOR: &str =
    "script, style, pre, svg, math, noscript, ref, table, tr, td, ol, ul, li, time, [aria-hidden], img, figure";
pub const DEFAULT_NL_APPEND_SELECTOR: &str = "div, p, hr, br, h1, h2, h3, h4, h5, h6";

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlCleaning {
    #[clap(name = "html.enabled", short = 'c', help = "Clean HTML tags")]
    pub enabled: bool,
    #[clap(
        long,
        help = "CSS selector for main content",
        requires = "html.enabled"
    )]
    content_selector: Option<String>,
    #[clap(long, default_value = DEFAULT_DELETE_SELECTOR, help="CSS selector for tag removal")]
    delete_selector: String,
    #[clap(long, default_value = DEFAULT_NL_APPEND_SELECTOR, help="CSS selector to append newline")]
    nl_append_selector: String,
}

impl Default for HtmlCleaning {
    fn default() -> HtmlCleaning {
        HtmlCleaning {
            enabled: true,
            content_selector: None,
            delete_selector: DEFAULT_DELETE_SELECTOR.to_owned(),
            nl_append_selector: DEFAULT_NL_APPEND_SELECTOR.to_owned(),
        }
    }
}

pub fn run(text: String, opts: HtmlCleaning) -> String {
    let text = clean_control(text); // html5ever panics if control chars are present
    let document = kuchiki::parse_html().one(text);

    let document = if opts.content_selector.is_some() {
        let content_selector = opts.content_selector.unwrap();
        document
            .select_first(&content_selector)
            .expect("Content selector had no matches")
            .as_node()
            .to_owned()
    } else {
        document
    };

    let nodes_to_delete = document
        .select(&opts.delete_selector)
        .unwrap()
        .collect::<Vec<NodeDataRef<ElementData>>>();

    for node in nodes_to_delete {
        node.as_node().detach();
    }

    for node in document.select(&opts.nl_append_selector).unwrap() {
        node.as_node()
            .insert_after(kuchiki::NodeRef::new_text(LINE_SEPARATOR));
    }

    document.text_contents()
}
