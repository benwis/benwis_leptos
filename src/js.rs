use crate::errors::BenwisAppError;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Recreate HtmlOutput so we don't have to import it from server only crates
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HtmlOutput {
    pub toc: Option<String>,
    pub content: String,
    pub frontmatter: Option<OwnedFrontmatter>,
}

/// An owned version of the CodeBlock used for the frontmatter. Makes it much easier to return.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OwnedCodeBlock {
    pub language: Option<String>,
    pub source: String,
}

/// An owned version of the Frontmatter, returned from our functions
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OwnedFrontmatter {
    pub title: Option<String>,
    pub code_block: Option<OwnedCodeBlock>,
}

#[wasm_bindgen(raw_module = "/components/femark-wasi/femark.js")]
extern "C" {
    pub fn processMarkdownToHtml(input: String) -> JsValue;

    pub fn processMarkdownToHtmlWithFrontmatter(input: String) -> JsValue;
}

pub fn process_markdown_to_html(input: String) -> Result<HtmlOutput, BenwisAppError> {
    let jsval = processMarkdownToHtml(input);
    Ok(serde_wasm_bindgen::from_value(jsval)?)
}

pub fn process_markdown_to_html_with_frontmatter(
    input: String,
) -> Result<HtmlOutput, BenwisAppError> {
    let jsval = processMarkdownToHtmlWithFrontmatter(input);
    Ok(serde_wasm_bindgen::from_value(jsval)?)
}
